//! Bounded frame IO on byte-mode streams (Design02 §4).
//!
//! Header reads are idle-tolerant; once the length header has arrived the
//! body must complete within `FRAME_BODY_DEADLINE`, and every write is
//! bounded by `WRITE_DEADLINE`. The length header is validated before any
//! payload allocation.

use autoos_comm_core::codec::check_frame_header;
use autoos_comm_core::error::CommError;
use autoos_comm_core::Limits;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::time::timeout;

/// Read one length-prefixed frame. `Ok(None)` = clean EOF between frames.
pub async fn read_frame(r: &mut (impl AsyncReadExt + Unpin)) -> Result<Option<Vec<u8>>, CommError> {
    let mut header = [0u8; 4];
    match r.read(&mut header).await {
        Ok(0) => return Ok(None),
        Ok(1..=3) => {
            // partial header: bounded tail read
            let mut got = r.read(&mut header).await.map_err(io_err)?;
            while got < 4 - 0 {
                let n = r.read(&mut header[got..]).await.map_err(io_err)?;
                if n == 0 {
                    return Err(CommError::invalid_frame("stream ended mid-header"));
                }
                got += n;
            }
        }
        Ok(_) => {}
        Err(e) => return Err(io_err(e)),
    }
    let len = u32::from_le_bytes(header);
    let size = check_frame_header(len)?;
    let mut payload = vec![0u8; size]; // allocated only after header validation
    timeout(Limits::FRAME_BODY_DEADLINE, r.read_exact(&mut payload))
        .await
        .map_err(|_| {
            CommError::deadline(
                autoos_comm_core::ExecutionState::NotDispatched,
                "frame body did not complete in time",
            )
        })?
        .map_err(io_err)?;
    Ok(Some(payload))
}

/// Write one already-framed byte buffer with a hard deadline.
pub async fn write_frame(
    w: &mut (impl AsyncWriteExt + Unpin),
    frame: &[u8],
    deadline: std::time::Duration,
) -> Result<(), CommError> {
    timeout(deadline, w.write_all(frame))
        .await
        .map_err(|_| {
            CommError::deadline(
                autoos_comm_core::ExecutionState::MayHaveRun,
                "peer did not read in time",
            )
        })?
        .map_err(io_err)?;
    timeout(deadline, w.flush())
        .await
        .map_err(|_| {
            CommError::deadline(
                autoos_comm_core::ExecutionState::MayHaveRun,
                "peer did not drain in time",
            )
        })?
        .map_err(io_err)
}

fn io_err(e: std::io::Error) -> CommError {
    CommError::disconnected(
        autoos_comm_core::ExecutionState::MayHaveRun,
        format!("io error: {e}"),
    )
}
