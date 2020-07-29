use std::os::raw::{c_char, c_int};

extern "C" {
    fn tunnel_open(tun_name: *const c_char) -> c_int;
    fn tunnel_get_name(fd: c_int, name: *mut c_char, maxlen: c_int) -> c_int;
    fn tunnel_set_hwaddr(fd: c_int, addr: *mut u8, addr_len: c_int) -> c_int;
    fn tunnel_close(fd: c_int) -> ();
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
