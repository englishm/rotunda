use std::ffi::{CStr, CString};
use std::fs::File;
use std::io::prelude::*;
use std::io::{Error, ErrorKind};
use std::os::raw::{c_char, c_int};
use std::os::unix::io::{AsRawFd, FromRawFd, IntoRawFd, RawFd};

extern "C" {
    fn tunnel_open(tun_name: *const c_char) -> c_int;
    fn tunnel_get_name(fd: c_int, name: *mut c_char, maxlen: c_int) -> c_int;
// fn tunnel_set_hwaddr(fd: c_int, addr: *mut u8, addr_len: c_int) -> c_int;
// fn tunnel_close(fd: c_int) -> ();
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub enum Mode {
    /// TUN mode
    ///
    /// Sends and receives packets on the IP layer (layer 3).
    /// This is the only supported mode.
    Tun = 1,
}

#[derive(Debug)]
pub struct Iface {
    fd: File,
    mode: Mode,
    name: String,
}

impl Iface {
    pub fn new(ifname: &str, mode: Mode) -> Result<Self, Error> {
        Iface::with_options(ifname, mode, true)
    }
    pub fn without_packet_info(ifname: &str, mode: Mode) -> Result<Self, Error> {
        Iface::with_options(ifname, mode, false)
    }

    fn with_options(ifname: &str, mode: Mode, packet_info: bool) -> Result<Self, Error> {
        if packet_info {
            return Err(Error::new(
                ErrorKind::Other,
                "packet info not yet supported",
            ));
        }
        let c_ifname = CString::new(ifname).expect("CString::new failed");
        let ifname_ptr = c_ifname.as_ptr();
        let fd = unsafe { File::from_raw_fd(tunnel_open(ifname_ptr)) };

        let mut name_buffer = Vec::new();
        const MAX_LEN: c_int = 32;
        name_buffer.extend_from_slice(ifname.as_bytes());
        name_buffer.extend_from_slice(&[0; MAX_LEN as usize]);
        let name_ptr = name_buffer.as_mut_ptr() as *mut c_char;
        let result = unsafe { tunnel_get_name(fd.as_raw_fd(), name_ptr, MAX_LEN) };
        if result < 0 {
            return Err(Error::last_os_error());
        }
        let name = unsafe {
            CStr::from_ptr(name_ptr as *const c_char)
                .to_string_lossy()
                .into_owned()
        };
        Ok(Iface { fd, mode, name })
    }
    pub fn mode(&self) -> Mode {
        self.mode
    }
    pub fn name(&self) -> &str {
        &self.name
    }
    pub fn recv(&self, buf: &mut [u8]) -> Result<usize, Error> {
        (&self.fd).read(buf)
    }
    pub fn send(&self, buf: &[u8]) -> Result<usize, Error> {
        (&self.fd).write(buf)
    }
}

impl AsRawFd for Iface {
    fn as_raw_fd(&self) -> RawFd {
        self.fd.as_raw_fd()
    }
}

impl IntoRawFd for Iface {
    fn into_raw_fd(self) -> RawFd {
        self.fd.into_raw_fd()
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
