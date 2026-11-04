//! Minimal Job-Object process containment for the test/demo controller.
//!
//! Pattern (reused from the private ac-core JobGuard experience, rebuilt
//! standalone so no Cranelift/language deps are pulled in): create the child
//! SUSPENDED, assign it to a kill-on-close Job, then resume. If anything
//! fails before the resume, the process is terminated — a failure never
//! degrades to an unmanaged run. This manages only the processes this
//! controller started; it is not a general process-management framework.

use std::ffi::c_void;
use std::path::Path;
use std::time::Duration;

use windows_sys::Win32::Foundation::{CloseHandle, GetLastError, HANDLE, WAIT_OBJECT_0};
use windows_sys::Win32::Security::SECURITY_ATTRIBUTES;
use windows_sys::Win32::System::JobObjects::{
    AssignProcessToJobObject, CreateJobObjectW, JobObjectExtendedLimitInformation,
    SetInformationJobObject, TerminateJobObject, JOBOBJECT_EXTENDED_LIMIT_INFORMATION,
    JOB_OBJECT_LIMIT_KILL_ON_JOB_CLOSE,
};
use windows_sys::Win32::System::Threading::{
    CreateProcessW, GetExitCodeProcess, ResumeThread, TerminateProcess, WaitForSingleObject,
    PROCESS_INFORMATION, STARTF_USESTDHANDLES, STARTUPINFOW,
};

const CREATE_SUSPENDED: u32 = 0x0000_0004;
const INFINITE: u32 = 0xFFFF_FFFF;

/// Optional child stdio handles (must already be inheritable).
#[derive(Default)]
pub struct ChildStdio {
    pub stdin: Option<HANDLE>,
    pub stdout: Option<HANDLE>,
    pub stderr: Option<HANDLE>,
}

pub struct ManagedChild {
    pub pid: u32,
    process: HANDLE,
    thread: HANDLE,
}

impl ManagedChild {
    /// Exit code once exited (`None` while running).
    pub fn try_wait(&self) -> std::io::Result<Option<u32>> {
        unsafe {
            let wait = WaitForSingleObject(self.process, 0);
            if wait != WAIT_OBJECT_0 {
                return Ok(None);
            }
            let mut code = 0u32;
            if GetExitCodeProcess(self.process, &mut code) == 0 {
                return Err(std::io::Error::last_os_error());
            }
            Ok(Some(code))
        }
    }

    pub fn wait(&self) -> std::io::Result<u32> {
        unsafe {
            WaitForSingleObject(self.process, INFINITE);
            let mut code = 0u32;
            if GetExitCodeProcess(self.process, &mut code) == 0 {
                return Err(std::io::Error::last_os_error());
            }
            Ok(code)
        }
    }

    /// Wait up to `d`, then `Ok(None)` if still running.
    pub fn wait_for(&self, d: Duration) -> std::io::Result<Option<u32>> {
        unsafe {
            let wait =
                WaitForSingleObject(self.process, d.as_millis().min(u32::MAX as u128) as u32);
            if wait != WAIT_OBJECT_0 {
                return Ok(None);
            }
            let mut code = 0u32;
            if GetExitCodeProcess(self.process, &mut code) == 0 {
                return Err(std::io::Error::last_os_error());
            }
            Ok(Some(code))
        }
    }

    pub fn kill(&self) {
        unsafe {
            TerminateProcess(self.process, 1);
            let _ = WaitForSingleObject(self.process, 5000);
        }
    }
}

impl Drop for ManagedChild {
    fn drop(&mut self) {
        unsafe {
            TerminateProcess(self.process, 1); // safety net; job also kills
            let _ = WaitForSingleObject(self.process, 2000);
            CloseHandle(self.thread);
            CloseHandle(self.process);
        }
    }
}

pub struct JobGuard {
    job: HANDLE,
}

impl JobGuard {
    pub fn new() -> std::io::Result<Self> {
        unsafe {
            let job = CreateJobObjectW(std::ptr::null(), std::ptr::null());
            if job.is_null() {
                return Err(std::io::Error::last_os_error());
            }
            let mut info: JOBOBJECT_EXTENDED_LIMIT_INFORMATION = std::mem::zeroed();
            info.BasicLimitInformation.LimitFlags = JOB_OBJECT_LIMIT_KILL_ON_JOB_CLOSE;
            let ok = SetInformationJobObject(
                job,
                JobObjectExtendedLimitInformation,
                &info as *const _ as *const c_void,
                std::mem::size_of::<JOBOBJECT_EXTENDED_LIMIT_INFORMATION>() as u32,
            );
            if ok == 0 {
                let e = std::io::Error::last_os_error();
                CloseHandle(job);
                return Err(e);
            }
            Ok(JobGuard { job })
        }
    }

    /// Spawn `exe` with `args` SUSPENDED, assign to this Job, then resume.
    /// On any failure the suspended child is terminated: never an unmanaged
    /// or half-managed run.
    pub fn spawn(
        &self,
        exe: &Path,
        args: &[&str],
        envs: &[(String, String)],
        cwd: Option<&Path>,
        stdio: ChildStdio,
    ) -> std::io::Result<ManagedChild> {
        let mut cmdline: Vec<u16> = Vec::new();
        push_quoted(&mut cmdline, &exe.to_string_lossy());
        for a in args {
            cmdline.push(' ' as u16);
            push_quoted(&mut cmdline, a);
        }
        cmdline.push(0);

        let mut env_block: Vec<u16> = Vec::new();
        for (k, v) in envs {
            env_block.extend(format!("{k}={v}").encode_utf16());
            env_block.push(0);
        }
        env_block.push(0);
        let env_ptr = if env_block.len() > 1 {
            Some(env_block.as_ptr())
        } else {
            None
        };

        let cwd_wide: Option<Vec<u16>> = cwd.map(|p| {
            let mut v: Vec<u16> = p.to_string_lossy().encode_utf16().collect();
            v.push(0);
            v
        });

        let inherit = stdio.stdin.is_some() || stdio.stdout.is_some() || stdio.stderr.is_some();
        let mut si: STARTUPINFOW = unsafe { std::mem::zeroed() };
        si.cb = std::mem::size_of::<STARTUPINFOW>() as u32;
        if inherit {
            si.dwFlags = STARTF_USESTDHANDLES;
            si.hStdInput = stdio.stdin.unwrap_or(std::ptr::null_mut());
            si.hStdOutput = stdio.stdout.unwrap_or(std::ptr::null_mut());
            si.hStdError = stdio.stderr.unwrap_or(std::ptr::null_mut());
        }
        let mut pi: PROCESS_INFORMATION = unsafe { std::mem::zeroed() };

        unsafe {
            let ok = CreateProcessW(
                std::ptr::null(),
                cmdline.as_mut_ptr(),
                std::ptr::null(),
                std::ptr::null(),
                inherit as i32,
                CREATE_SUSPENDED,
                env_ptr.unwrap_or(std::ptr::null()) as *const c_void,
                cwd_wide
                    .as_ref()
                    .map(|v| v.as_ptr())
                    .unwrap_or(std::ptr::null()),
                &si,
                &mut pi,
            );
            if ok == 0 {
                return Err(std::io::Error::last_os_error());
            }
            if AssignProcessToJobObject(self.job, pi.hProcess) == 0 {
                let e = std::io::Error::last_os_error();
                TerminateProcess(pi.hProcess, 1);
                WaitForSingleObject(pi.hProcess, 5000);
                CloseHandle(pi.hThread);
                CloseHandle(pi.hProcess);
                return Err(e);
            }
            if ResumeThread(pi.hThread) == u32::MAX {
                let e = std::io::Error::last_os_error();
                TerminateProcess(pi.hProcess, 1);
                WaitForSingleObject(pi.hProcess, 5000);
                CloseHandle(pi.hThread);
                CloseHandle(pi.hProcess);
                return Err(e);
            }
            Ok(ManagedChild {
                pid: pi.dwProcessId,
                process: pi.hProcess,
                thread: pi.hThread,
            })
        }
    }
}

fn push_quoted(out: &mut Vec<u16>, s: &str) {
    out.push('"' as u16);
    for c in s.encode_utf16() {
        if c == '"' as u16 {
            out.push('\\' as u16);
        }
        out.push(c);
    }
    out.push('"' as u16);
}

/// Terminate every process currently in the job: direct PID enumeration
/// first (robust under nested/sandboxed jobs), TerminateJobObject as the
/// catch-all.
pub fn terminate_job(job: HANDLE) {
    const PROCESS_TERMINATE: u32 = 0x0001;
    unsafe {
        use windows_sys::Win32::System::JobObjects::{
            JobObjectBasicProcessIdList, QueryInformationJobObject,
        };
        // 8 header bytes + up to 64 ULONG_PTR pids
        let mut buf = [0u8; 16 + 64 * std::mem::size_of::<usize>()];
        let ok = QueryInformationJobObject(
            job,
            JobObjectBasicProcessIdList,
            buf.as_mut_ptr() as *const _ as *mut c_void,
            buf.len() as u32,
            std::ptr::null_mut(),
        );
        if ok != 0 {
            let in_list = u32::from_le_bytes([buf[4], buf[5], buf[6], buf[7]]) as usize;
            for i in 0..in_list.min(64) {
                let off = 8 + i * std::mem::size_of::<usize>();
                let pid = usize::from_le_bytes(buf[off..off + 8].try_into().unwrap());
                if pid == 0 {
                    continue;
                }
                let h = windows_sys::Win32::System::Threading::OpenProcess(
                    PROCESS_TERMINATE,
                    0,
                    pid as u32,
                );
                if !h.is_null() {
                    TerminateProcess(h, 1);
                    CloseHandle(h);
                }
            }
        }
        TerminateJobObject(job, 0);
    }
}

impl Drop for JobGuard {
    fn drop(&mut self) {
        unsafe {
            terminate_job(self.job);
            CloseHandle(self.job); // kill-on-close as the final safety net
        }
    }
}

/// True if the process `pid` no longer exists (used for reclaim checks).
pub fn process_gone(pid: u32) -> bool {
    const SYNCHRONIZE: u32 = 0x0010_0000; // standard access right (winnt.h)
    use windows_sys::Win32::System::Threading::{OpenProcess, PROCESS_QUERY_LIMITED_INFORMATION};
    unsafe {
        // SYNCHRONIZE is required for WaitForSingleObject on the handle
        let h = OpenProcess(PROCESS_QUERY_LIMITED_INFORMATION | SYNCHRONIZE, 0, pid);
        if h.is_null() {
            return true;
        }
        let wait = WaitForSingleObject(h, 0);
        CloseHandle(h);
        wait == WAIT_OBJECT_0 // signaled => exited
    }
}

/// Create an anonymous pipe with an inheritable write end (for tests that
/// must observe the child's stdout, including descendants holding it).
pub fn child_stdout_pipe() -> std::io::Result<(std::fs::File, HANDLE)> {
    use windows_sys::Win32::Foundation::SetHandleInformation;
    use windows_sys::Win32::Foundation::HANDLE_FLAG_INHERIT;
    use windows_sys::Win32::System::Pipes::CreatePipe;
    let mut sa: SECURITY_ATTRIBUTES = unsafe { std::mem::zeroed() };
    sa.nLength = std::mem::size_of::<SECURITY_ATTRIBUTES>() as u32;
    sa.bInheritHandle = 1;
    let mut read_end: HANDLE = std::ptr::null_mut();
    let mut write_end: HANDLE = std::ptr::null_mut();
    unsafe {
        if CreatePipe(&mut read_end, &mut write_end, &sa, 0) == 0 {
            return Err(std::io::Error::last_os_error());
        }
        if SetHandleInformation(write_end, HANDLE_FLAG_INHERIT, HANDLE_FLAG_INHERIT) == 0 {
            let e = std::io::Error::last_os_error();
            CloseHandle(read_end);
            CloseHandle(write_end);
            return Err(e);
        }
        use std::os::windows::io::FromRawHandle;
        Ok((std::fs::File::from_raw_handle(read_end), write_end))
    }
}

/// Best-effort last-error report used in diagnostics (never token material).
pub fn last_error_note() -> u32 {
    unsafe { GetLastError() }
}
