//! Named-pipe server/client with explicit DACL and DACL inspection.
//!
//! The default named-pipe DACL grants Everyone/Anonymous read access
//! (Microsoft docs, "Named Pipe Security and Access Rights"), so every pipe
//! is created with a protected explicit DACL granting only the current user
//! and SYSTEM the necessary read/write access, with remote clients rejected.
//! `pipe_dacl_sids` lets tests verify the actual handle DACL.

use crate::frame_io::{read_frame, write_frame};
use autoos_comm_core::error::CommError;
use autoos_comm_core::protocol::{Error as ErrorMessage, Message, CODEC_ID, PROTOCOL_VERSION};
use std::ffi::c_void;
use std::time::Duration;
use tokio::net::windows::named_pipe::{ClientOptions, NamedPipeClient, NamedPipeServer};
use tokio::time::timeout;
use windows_sys::Win32::Foundation::{CloseHandle, LocalFree, HANDLE};
use windows_sys::Win32::Security::Authorization::{
    ConvertSidToStringSidW, ConvertStringSidToSidW, GetSecurityInfo, SetEntriesInAclW,
    EXPLICIT_ACCESS_W, GRANT_ACCESS, SE_KERNEL_OBJECT, TRUSTEE_IS_SID, TRUSTEE_IS_USER,
    TRUSTEE_IS_WELL_KNOWN_GROUP, TRUSTEE_W,
};
use windows_sys::Win32::Security::{
    AclSizeInformation, GetAce, GetAclInformation, GetLengthSid, GetTokenInformation,
    InitializeSecurityDescriptor, SetSecurityDescriptorDacl, TokenUser, ACCESS_ALLOWED_ACE, ACL,
    ACL_SIZE_INFORMATION, DACL_SECURITY_INFORMATION, SECURITY_ATTRIBUTES, SECURITY_DESCRIPTOR,
    TOKEN_QUERY, TOKEN_USER,
};
// SECURITY_DESCRIPTOR_REVISION is the documented value 1 (winnt.h); windows-sys
// only exposes it behind an unrelated feature gate.
const SECURITY_DESCRIPTOR_REVISION: u32 = 1;
use windows_sys::Win32::Storage::FileSystem::{
    FILE_FLAG_FIRST_PIPE_INSTANCE, FILE_FLAG_OVERLAPPED, FILE_GENERIC_READ, FILE_GENERIC_WRITE,
    PIPE_ACCESS_DUPLEX,
};
use windows_sys::Win32::System::Pipes::{
    PIPE_REJECT_REMOTE_CLIENTS, PIPE_TYPE_BYTE, PIPE_UNLIMITED_INSTANCES,
};
use windows_sys::Win32::System::Threading::{GetCurrentProcess, OpenProcessToken};

// windows-sys omits the zero-valued pipe mode constants; these are the
// documented winbase values (PIPE_READMODE_BYTE = 0, PIPE_WAIT = 0).
const PIPE_READMODE_BYTE: u32 = 0;
const PIPE_WAIT: u32 = 0;

/// Random experimental pipe namespace (`\\.\pipe\autoos-comm-f01-<random>`).
/// The broker never listens on a fixed well-known name (Design02 §3).
pub struct PipeName;

impl PipeName {
    pub fn new_random() -> String {
        format!(
            r"\\.\pipe\autoos-comm-f01-{}",
            autoos_comm_core::ids::new_id()
        )
    }
}

const SYSTEM_SID_STRING: &[u16] = &[
    b'S' as u16,
    b'-' as u16,
    b'1' as u16,
    b'-' as u16,
    b'5' as u16,
    b'-' as u16,
    b'1' as u16,
    b'8' as u16,
    0,
];

fn last_os_error() -> std::io::Error {
    std::io::Error::last_os_error()
}

fn current_user_sid() -> Result<Vec<u8>, std::io::Error> {
    unsafe {
        let mut token: HANDLE = std::ptr::null_mut();
        if OpenProcessToken(GetCurrentProcess(), TOKEN_QUERY, &mut token) == 0 {
            return Err(last_os_error());
        }
        let mut len = 0u32;
        let _ = GetTokenInformation(token, TokenUser, std::ptr::null_mut(), 0, &mut len);
        let mut buf = vec![0u8; len as usize];
        let ok = GetTokenInformation(
            token,
            TokenUser,
            buf.as_mut_ptr() as *mut c_void,
            len,
            &mut len,
        );
        let err = last_os_error();
        CloseHandle(token);
        if ok == 0 {
            return Err(err);
        }
        let user = buf.as_ptr() as *const TOKEN_USER;
        let sid = (*user).User.Sid;
        let sid_len = GetLengthSid(sid) as usize;
        let sid_bytes = std::slice::from_raw_parts(sid as *const u8, sid_len).to_vec();
        Ok(sid_bytes)
    }
}

/// Current user's SID in SDDL string form (used by tests to compare DACLs).
pub fn current_user_sid_string() -> Result<String, std::io::Error> {
    let bytes = current_user_sid()?;
    unsafe {
        let mut s: *mut u16 = std::ptr::null_mut();
        if ConvertSidToStringSidW(bytes.as_ptr() as *mut c_void, &mut s) == 0 {
            return Err(last_os_error());
        }
        let out = wide_to_string(s);
        LocalFree(s as _);
        Ok(out)
    }
}

unsafe fn wide_to_string(mut p: *const u16) -> String {
    let mut v = Vec::new();
    while *p != 0 {
        v.push(*p);
        p = p.add(1);
    }
    String::from_utf16_lossy(&v)
}

/// Build a protected explicit DACL granting current user + SYSTEM
/// generic read|write; nobody else gets anything (no inherited ACEs).
unsafe fn build_security_descriptor() -> Result<*mut SECURITY_DESCRIPTOR, std::io::Error> {
    let user_sid = current_user_sid()?;
    let mut system_sid: *mut std::ffi::c_void = std::ptr::null_mut();
    if ConvertStringSidToSidW(SYSTEM_SID_STRING.as_ptr(), &mut system_sid) == 0 {
        return Err(last_os_error());
    }
    let access = (FILE_GENERIC_READ | FILE_GENERIC_WRITE) as u32;
    let entries = [
        EXPLICIT_ACCESS_W {
            grfAccessPermissions: access,
            grfAccessMode: GRANT_ACCESS,
            grfInheritance: 0,
            Trustee: TRUSTEE_W {
                pMultipleTrustee: std::ptr::null_mut(),
                MultipleTrusteeOperation: 0,
                TrusteeForm: TRUSTEE_IS_SID,
                TrusteeType: TRUSTEE_IS_USER,
                ptstrName: user_sid.as_ptr() as *mut u16,
            },
        },
        EXPLICIT_ACCESS_W {
            grfAccessPermissions: access,
            grfAccessMode: GRANT_ACCESS,
            grfInheritance: 0,
            Trustee: TRUSTEE_W {
                pMultipleTrustee: std::ptr::null_mut(),
                MultipleTrusteeOperation: 0,
                TrusteeForm: TRUSTEE_IS_SID,
                TrusteeType: TRUSTEE_IS_WELL_KNOWN_GROUP,
                ptstrName: system_sid as *mut u16,
            },
        },
    ];
    let mut acl: *mut ACL = std::ptr::null_mut();
    let rc = SetEntriesInAclW(
        entries.len() as u32,
        entries.as_ptr(),
        std::ptr::null(),
        &mut acl,
    );
    if rc != 0 {
        LocalFree(system_sid);
        return Err(std::io::Error::from_raw_os_error(rc as i32));
    }
    LocalFree(system_sid);
    let sd = Box::into_raw(Box::new(std::mem::zeroed::<SECURITY_DESCRIPTOR>()));
    if InitializeSecurityDescriptor(sd as *mut c_void, SECURITY_DESCRIPTOR_REVISION) == 0 {
        let e = last_os_error();
        LocalFree(acl as _);
        drop(Box::from_raw(sd));
        return Err(e);
    }
    // SE_DACL_PRESENT | SE_DACL_PROTECTED: no inherited Everyone/Anonymous ACEs.
    unsafe {
        (*sd).Control |= 0x0004u16 | 0x1000u16;
    }
    if SetSecurityDescriptorDacl(sd as *mut c_void, 1, acl, 0) == 0 {
        let e = last_os_error();
        LocalFree(acl as _);
        drop(Box::from_raw(sd));
        return Err(e);
    }
    // Ownership of `acl` transfers into the SD; caller frees via free_security_descriptor.
    Ok(sd)
}

unsafe fn free_security_descriptor(sd: *mut SECURITY_DESCRIPTOR) {
    if sd.is_null() {
        return;
    }
    let mut dacl: *mut ACL = std::ptr::null_mut();
    let mut present = 0;
    let mut defaulted = 0;
    if windows_sys::Win32::Security::GetSecurityDescriptorDacl(
        sd as *mut c_void,
        &mut present,
        &mut dacl,
        &mut defaulted,
    ) != 0
        && present != 0
        && !dacl.is_null()
    {
        LocalFree(dacl as _);
    }
    drop(Box::from_raw(sd));
}

/// Create one server-side pipe instance with the explicit DACL.
pub fn create_pipe_server(name: &str, first_instance: bool) -> std::io::Result<NamedPipeServer> {
    let mut wide: Vec<u16> = name.encode_utf16().collect();
    wide.push(0);
    unsafe {
        let sd = build_security_descriptor()?;
        let sa = SECURITY_ATTRIBUTES {
            nLength: std::mem::size_of::<SECURITY_ATTRIBUTES>() as u32,
            lpSecurityDescriptor: sd as *mut c_void,
            bInheritHandle: 0,
        };
        let mut openmode = PIPE_ACCESS_DUPLEX | FILE_FLAG_OVERLAPPED;
        if first_instance {
            openmode |= FILE_FLAG_FIRST_PIPE_INSTANCE;
        }
        let handle = windows_sys::Win32::System::Pipes::CreateNamedPipeW(
            wide.as_ptr(),
            openmode,
            PIPE_TYPE_BYTE | PIPE_READMODE_BYTE | PIPE_WAIT | PIPE_REJECT_REMOTE_CLIENTS,
            PIPE_UNLIMITED_INSTANCES,
            65536 + 1024,
            65536 + 1024,
            0,
            &sa,
        );
        free_security_descriptor(sd);
        if handle as isize == -1 {
            // INVALID_HANDLE_VALUE
            return Err(last_os_error());
        }
        use std::os::windows::io::RawHandle;
        Ok(NamedPipeServer::from_raw_handle(handle as RawHandle)?)
    }
}

/// Connect one client end, retrying while the pipe instance is busy or not
/// yet created, until `deadline`.
pub async fn connect_pipe(
    name: &str,
    deadline: std::time::Instant,
) -> std::io::Result<NamedPipeClient> {
    loop {
        if std::time::Instant::now() >= deadline {
            return Err(std::io::Error::new(
                std::io::ErrorKind::TimedOut,
                "pipe connect deadline passed",
            ));
        }
        match ClientOptions::new().open(name) {
            Ok(c) => return Ok(c),
            Err(e) if e.raw_os_error() == Some(2) || e.raw_os_error() == Some(231) => {
                // FILE_NOT_FOUND / PIPE_BUSY: retry shortly
                tokio::time::sleep(Duration::from_millis(20)).await;
            }
            Err(e) => return Err(e),
        }
    }
}

/// Read the actual DACL SIDs of a pipe handle (V-05 oracle).
pub fn pipe_dacl_sids(handle: HANDLE) -> Result<Vec<String>, std::io::Error> {
    unsafe {
        let mut dacl: *mut ACL = std::ptr::null_mut();
        let mut sd: *mut c_void = std::ptr::null_mut();
        let rc = GetSecurityInfo(
            handle,
            SE_KERNEL_OBJECT,
            DACL_SECURITY_INFORMATION,
            std::ptr::null_mut(),
            std::ptr::null_mut(),
            &mut dacl,
            std::ptr::null_mut(),
            &mut sd,
        );
        if rc != 0 {
            return Err(std::io::Error::from_raw_os_error(rc as i32));
        }
        let mut info: ACL_SIZE_INFORMATION = std::mem::zeroed();
        if GetAclInformation(
            dacl,
            &mut info as *mut _ as *mut c_void,
            std::mem::size_of::<ACL_SIZE_INFORMATION>() as u32,
            AclSizeInformation,
        ) == 0
        {
            let e = last_os_error();
            LocalFree(sd);
            return Err(e);
        }
        let mut sids = Vec::new();
        for i in 0..info.AceCount {
            let mut ace: *mut c_void = std::ptr::null_mut();
            if GetAce(dacl, i, &mut ace) == 0 {
                let e = last_os_error();
                LocalFree(sd);
                return Err(e);
            }
            let allowed = ace as *mut ACCESS_ALLOWED_ACE;
            if (*allowed).Header.AceType == 0 {
                // ACCESS_ALLOWED_ACE_TYPE; SidStart marks the SID start
                let sid_ptr = &(*allowed).SidStart as *const u32 as *const u8;
                let sid_len = GetLengthSid(sid_ptr as _) as usize;
                let sid_bytes = std::slice::from_raw_parts(sid_ptr, sid_len);
                let mut s: *mut u16 = std::ptr::null_mut();
                if ConvertSidToStringSidW(sid_bytes.as_ptr() as *mut c_void, &mut s) == 0 {
                    let e = last_os_error();
                    LocalFree(sd);
                    return Err(e);
                }
                sids.push(wide_to_string(s));
                LocalFree(s as _);
            }
        }
        LocalFree(sd);
        Ok(sids)
    }
}

/// Query the PIPE_REJECT_REMOTE_CLIENTS state bit of a pipe handle
/// (V-05 evidence: the flag is set on the real object, not just requested).
pub fn pipe_rejects_remote(handle: HANDLE) -> std::io::Result<bool> {
    unsafe {
        use windows_sys::Win32::System::Pipes::GetNamedPipeInfo;
        let mut flags: u32 = 0;
        let mut out_size: u32 = 0;
        let mut in_size: u32 = 0;
        let mut max_instances: u32 = 0;
        if GetNamedPipeInfo(
            handle,
            &mut flags,
            &mut out_size,
            &mut in_size,
            &mut max_instances,
        ) == 0
        {
            return Err(last_os_error());
        }
        // PIPE_REJECT_REMOTE_CLIENTS (0x8); 0 means PIPE_ACCEPT_REMOTE_CLIENTS
        Ok(flags & PIPE_REJECT_REMOTE_CLIENTS != 0)
    }
}

/// Convenience wrapper: raw `HANDLE` -> `HANDLE`-based DACL probe for tests
/// that hold a raw server handle before wrapping.
pub fn pipe_dacl_sids_raw(handle: isize) -> Result<Vec<String>, std::io::Error> {
    pipe_dacl_sids(handle as HANDLE)
}

/// Server-side Hello exchange. Returns the Welcome to send, or an error
/// message to send before closing. Credentials are never echoed or logged.
pub async fn server_handshake(
    server: &mut NamedPipeServer,
    epoch: &str,
    policy: &crate::policy::AccessPolicy,
    handshake_deadline: Duration,
) -> Result<crate::policy::Session, CommError> {
    let started = std::time::Instant::now();
    let frame = timeout(handshake_deadline, read_frame(server))
        .await
        .map_err(|_| {
            CommError::deadline(
                autoos_comm_core::ExecutionState::NotDispatched,
                "handshake deadline",
            )
        })??;
    let frame = frame.ok_or_else(|| {
        CommError::disconnected(
            autoos_comm_core::ExecutionState::NotDispatched,
            "peer closed before Hello",
        )
    })?;
    let _ = started;
    let msg = autoos_comm_core::codec::decode(&frame)?;
    let hello = match msg {
        Message::Hello(h) => h,
        other => {
            return Err(CommError::schema(format!(
                "expected hello, got `{}`",
                other.op_name()
            )))
        }
    };
    if hello.protocol_version != PROTOCOL_VERSION {
        let e = ErrorMessage::from_comm(
            None,
            None,
            None,
            CommError::version(format!(
                "protocol version {} unsupported",
                hello.protocol_version
            )),
        );
        let _ = send_message(server, &Message::Error(e)).await;
        return Err(CommError::version("peer protocol version mismatch"));
    }
    if hello.codec != CODEC_ID {
        let e = ErrorMessage::from_comm(
            None,
            None,
            None,
            CommError::version(format!("codec `{}` unsupported", hello.codec)),
        );
        let _ = send_message(server, &Message::Error(e)).await;
        return Err(CommError::version("peer codec mismatch"));
    }
    let session = match policy.authenticate(&hello.token) {
        Some(s) => s,
        None => {
            let e = ErrorMessage::from_comm(
                None,
                None,
                None,
                CommError::permission("unknown credential"),
            );
            let _ = send_message(server, &Message::Error(e)).await;
            return Err(CommError::permission("unknown credential"));
        }
    };
    let welcome = Message::Welcome(autoos_comm_core::protocol::Welcome {
        broker_epoch: epoch.to_string(),
        principal: session.principal.clone(),
        role: session.role.clone(),
    });
    send_message(server, &welcome).await?;
    Ok(session)
}

/// Encode + send one message frame with the write deadline.
pub async fn send_message(
    w: &mut (impl tokio::io::AsyncWriteExt + Unpin),
    msg: &Message,
) -> Result<(), CommError> {
    let bytes = autoos_comm_core::codec::encode(msg)?;
    write_frame(w, &bytes, autoos_comm_core::Limits::WRITE_DEADLINE).await
}

/// Read + decode one message frame. `Ok(None)` on clean EOF between frames.
pub async fn read_message(
    r: &mut (impl tokio::io::AsyncReadExt + Unpin),
) -> Result<Option<Message>, CommError> {
    match read_frame(r).await? {
        None => Ok(None),
        Some(bytes) => autoos_comm_core::codec::decode(&bytes).map(Some),
    }
}
