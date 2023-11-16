#[cfg(not(windows))]
use std::mem;
use std::time::Duration;

#[derive(Debug, thiserror::Error)]
#[cfg_attr(target_os = "windows", allow(unused))]
pub enum Error {
    #[cfg(target_os = "linux")]
    #[error("sysinfo failed")]
    Sysinfo,

    #[cfg(any(
        target_os = "macos",
        target_os = "freebsd",
        target_os = "openbsd",
        target_os = "netbsd"
    ))]
    #[error("sysctl failed")]
    Sysctl,

    #[cfg(any(
        target_os = "macos",
        target_os = "freebsd",
        target_os = "openbsd",
        target_os = "netbsd"
    ))]
    #[error(transparent)]
    SystemTime(#[from] std::time::SystemTimeError),
}

#[cfg(target_os = "linux")]
pub fn get() -> Result<Duration, Error> {
    let mut info: libc::sysinfo = unsafe { mem::zeroed() };
    let ret = unsafe { libc::sysinfo(&mut info) };
    if ret == 0 {
        Ok(Duration::from_secs(info.uptime as u64))
    } else {
        Err(Error::Sysinfo)
    }
}

#[cfg(any(
    target_os = "macos",
    target_os = "freebsd",
    target_os = "openbsd",
    target_os = "netbsd"
))]
pub fn get() -> Result<Duration, Error> {
    use std::time::SystemTime;
    let mut request = [libc::CTL_KERN, libc::KERN_BOOTTIME];
    let mut boottime: libc::timeval = unsafe { mem::zeroed() };
    let mut size: libc::size_t = mem::size_of_val(&boottime) as libc::size_t;
    let ret = unsafe {
        libc::sysctl(
            &mut request[0],
            2,
            &mut boottime as *mut libc::timeval as *mut libc::c_void,
            &mut size,
            std::ptr::null_mut(),
            0,
        )
    };
    if ret == 0 {
        Ok({
            SystemTime::now().duration_since(SystemTime::UNIX_EPOCH)?
                - Duration::new(boottime.tv_sec as u64, boottime.tv_usec as u32 * 1000)
        })
    } else {
        Err(Error::Sysctl)
    }
}

#[cfg(target_os = "windows")]
pub fn get() -> Result<Duration, Error> {
    let ret: u64 = unsafe { windows::Win32::System::SystemInformation::GetTickCount64() };
    Ok(Duration::from_millis(ret))
}
