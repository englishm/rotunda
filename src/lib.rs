use std::os::raw::{c_char, c_int};
use std::ffi::{CStr, CString};
use std::fs::File;
use std::io::prelude::*;
use std::os::unix::io::FromRawFd;
use std::io::Error;

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
    mode: Mode, /// For now, always Tun
    name: String,
}

impl Iface {
    pub fn new(ifname: &str, mode: Mode) -> Result<Self,&str> {
        Iface::with_options(ifname, mode, true)
    }
    pub fn without_packet_info(ifname: &str, mode: Mode) -> Result<Self,&str> {
        Iface::with_options(ifname, mode, false)
    }

    fn with_options(ifname: &str, mode: Mode, packet_info: bool) -> Result<Self,&str> {
        // let fd = OpenOptions::new()
        //     .read(true)
        //     .write(true)
        //     .open("/dev/net/tun")?;
        let tunnel_default_interface_name = CString::new("utun2").expect("CString::new failed");
        let ifname_ptr = tunnel_default_interface_name.as_ptr();
        let fd = unsafe {
            File::from_raw_fd(tunnel_open(ifname_ptr))
        };
        // // The buffer is larger than needed, but who cares… it is large enough.
        // let mut name_buffer = Vec::new();
        // name_buffer.extend_from_slice(ifname.as_bytes());
        // name_buffer.extend_from_slice(&[0; 33]);
        // let name_ptr: *mut u8 = name_buffer.as_mut_ptr();
        // let result = unsafe { tuntap_setup(fd.as_raw_fd(), name_ptr, mode as c_int, { if packet_info { 1 } else { 0 } }) };
        // if result < 0 {
        //     return Err(Error::last_os_error());
        // }
        if packet_info {
            return Err("Packet info is not yet supported")
        }
        // let name = unsafe {
        //     CStr::from_ptr(name_ptr as *const c_char)
        //         .to_string_lossy()
        //         .into_owned()
        // };

        let mut name_buffer = Vec::new();
        const max_len: c_int = 32;
        name_buffer.extend_from_slice(ifname.as_bytes());
        name_buffer.extend_from_slice(&[0; max_len]);
        let name_ptr: *mut u8 = name_buffer.as_mut_ptr();
        let result = unsafe {
            tunnel_get_name(fd.as_raw_fd(), name_ptr, max_len)
        };
        if result < 0 {
            return Err(Error::last_os_error());
        }
        let name = unsafe {
            CStr::from_ptr(name_ptr as *const c_char)
                .to_string_lossy()
                .into_owned()
        };
        Ok(Iface {
            fd,
            mode,
            name,
        })
    }
    pub fn mode(&self) -> Mode {
        self.mode
    }
    pub fn name(&self) -> &str {
        &self.name
    }
    pub fn recv(&self, buf: &mut [u8]) -> Result<usize,&str> {
        (&self.fd).read(buf)
    }
    pub fn send(&self, buf: &[u8]) -> Result<usize,&str> {
        (&self.fd).write(buf)
    }
}

// impl AsRawFd for Iface {
//     fn as_raw_fd(&self) -> RawFd {
//         self.fd.as_raw_fd()
//     }
// }

// impl IntoRawFd for Iface {
//     fn into_raw_fd(self) -> RawFd {
//         self.fd.into_raw_fd()
//     }
// }

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
