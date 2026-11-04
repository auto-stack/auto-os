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
        Ok(n @ 1..=3) => {
            // partial header: bounded completion of the remaining bytes
            let mut got = n;
            while got < 4 {
                let m = r.read(&mut header[got..]).await.map_err(io_err)?;
                if m == 0 {
                    return Err(CommError::invalid_frame("stream ended mid-header"));
                }
                got += m;
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::pin::Pin;
    use std::task::{Context, Poll};

    /// Yields the buffer one byte at a time to force split reads.
    struct DripReader {
        buf: std::io::Cursor<Vec<u8>>,
    }

    impl tokio::io::AsyncRead for DripReader {
        fn poll_read(
            mut self: Pin<&mut Self>,
            _cx: &mut Context<'_>,
            dst: &mut tokio::io::ReadBuf<'_>,
        ) -> Poll<std::io::Result<()>> {
            let pos = self.buf.position() as usize;
            let data = self.buf.get_ref();
            if pos >= data.len() {
                return Poll::Ready(Ok(()));
            }
            dst.put_slice(&data[pos..pos + 1]);
            self.buf.set_position((pos + 1) as u64);
            Poll::Ready(Ok(()))
        }
    }

    fn frame(payload: &[u8]) -> Vec<u8> {
        let mut v = (payload.len() as u32).to_le_bytes().to_vec();
        v.extend_from_slice(payload);
        v
    }

    #[tokio::test]
    async fn split_frame_reassembles_from_single_byte_reads() {
        let payload = br#"{"op":"discover","corr_id":1,"service_id":"auto.capture.inbox"}"#;
        let mut r = DripReader {
            buf: std::io::Cursor::new(frame(payload)),
        };
        let got = read_frame(&mut r).await.expect("split frame");
        assert_eq!(got.as_deref(), Some(&payload[..]));
        // clean EOF afterwards
        assert!(matches!(read_frame(&mut r).await, Ok(None)));
    }

    #[tokio::test]
    async fn coalesced_frames_read_one_per_call() {
        let mut two = frame(br#"{"op":"discover","corr_id":1,"service_id":"a"}"#);
        two.extend_from_slice(&frame(br#"{"op":"discover","corr_id":2,"service_id":"b"}"#));
        let mut r = DripReader {
            buf: std::io::Cursor::new(two),
        };
        let first = read_frame(&mut r).await.expect("first");
        assert_eq!(
            first.as_deref(),
            Some(&br#"{"op":"discover","corr_id":1,"service_id":"a"}"#[..])
        );
        let second = read_frame(&mut r).await.expect("second");
        assert_eq!(
            second.as_deref(),
            Some(&br#"{"op":"discover","corr_id":2,"service_id":"b"}"#[..])
        );
    }

    #[tokio::test]
    async fn truncated_header_and_body_are_bounded_errors() {
        // header only, stream ends: not a clean EOF (mid-frame)
        let mut r = DripReader {
            buf: std::io::Cursor::new(vec![4, 0, 0]),
        };
        let e = read_frame(&mut r).await.expect_err("truncated header");
        assert!(matches!(
            e.code,
            autoos_comm_core::error::ErrorCode::InvalidFrame
        ));
        // declared length 4 but body never arrives → deadline (bounded), no hang
        let mut r = DripReader {
            buf: std::io::Cursor::new(vec![4, 0, 0, 0]),
        };
        let e = read_frame(&mut r).await.expect_err("truncated body");
        // stream-end mid-body is bounded: either the deadline or the
        // disconnected classification — never a hang, never a fake success
        assert!(matches!(
            e.code,
            autoos_comm_core::error::ErrorCode::DeadlineExceeded
                | autoos_comm_core::error::ErrorCode::Disconnected
        ));
    }

    /// Never yields data and never closes: simulates a stalled peer.
    struct StallReader;

    impl tokio::io::AsyncRead for StallReader {
        fn poll_read(
            self: Pin<&mut Self>,
            _cx: &mut Context<'_>,
            _dst: &mut tokio::io::ReadBuf<'_>,
        ) -> Poll<std::io::Result<()>> {
            Poll::Pending
        }
    }

    /// Delivers one 4-byte header then stalls forever: the frame-body
    /// deadline must fire — a残帧 can never wedge the reader.
    struct HeaderThenStall {
        sent: bool,
    }

    impl tokio::io::AsyncRead for HeaderThenStall {
        fn poll_read(
            mut self: Pin<&mut Self>,
            cx: &mut Context<'_>,
            dst: &mut tokio::io::ReadBuf<'_>,
        ) -> Poll<std::io::Result<()>> {
            if !self.sent {
                self.sent = true;
                dst.put_slice(&[8, 0, 0, 0]);
                Poll::Ready(Ok(()))
            } else {
                cx.waker().wake_by_ref();
                Poll::Pending
            }
        }
    }

    #[tokio::test]
    async fn stalled_frame_body_hits_deadline() {
        let mut r = HeaderThenStall { sent: false };
        let e = read_frame(&mut r).await.expect_err("stalled body");
        assert!(
            matches!(e.code, autoos_comm_core::error::ErrorCode::DeadlineExceeded),
            "frame-body deadline must bound a残帧: {e:?}"
        );
    }

    #[tokio::test]
    async fn idle_between_frames_is_not_bounded() {
        // a connection that sends nothing at all is NOT killed by the frame
        // deadline (idle tolerance); the future simply stays pending. We
        // assert bounded non-completion via a short race window.
        let mut r = StallReader;
        let res =
            tokio::time::timeout(std::time::Duration::from_millis(200), read_frame(&mut r)).await;
        assert!(res.is_err(), "idle reader must not error on its own");
    }

    #[tokio::test]
    async fn zero_and_oversize_lengths_reject_before_allocation() {
        let mut r = DripReader {
            buf: std::io::Cursor::new(0u32.to_le_bytes().to_vec()),
        };
        let e = read_frame(&mut r).await.expect_err("zero length");
        assert!(matches!(
            e.code,
            autoos_comm_core::error::ErrorCode::InvalidFrame
        ));
        let mut r = DripReader {
            buf: std::io::Cursor::new(65537u32.to_le_bytes().to_vec()),
        };
        let e = read_frame(&mut r).await.expect_err("oversize");
        assert!(matches!(
            e.code,
            autoos_comm_core::error::ErrorCode::ResourceExhausted
        ));
    }
}
