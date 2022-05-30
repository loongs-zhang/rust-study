use std::ffi::c_void;
use std::io::Error;
use std::mem;
use libc as c;

fn main() {
    // client
    unsafe {
        let socket = c::socket(c::AF_INET, c::SOCK_STREAM, c::IPPROTO_TCP);
        if socket < 0 {
            panic!("last OS error: {:?}", Error::last_os_error());
        }

        let servaddr = c::sockaddr_in {
            sin_len: 0,
            sin_family: c::AF_INET as u8,
            sin_port: 9898u16.to_be(),
            sin_addr: c::in_addr {
                s_addr: u32::from_be_bytes([127, 0, 0, 1]).to_be()
            },
            sin_zero: mem::zeroed(),
        };

        let result = c::connect(socket, &servaddr as *const c::sockaddr_in as *const c::sockaddr, mem::size_of_val(&servaddr) as u32);
        if result < 0 {
            println!("last OS error: {:?}", Error::last_os_error());
            c::close(socket);
        }

        let msg = b"Hello, server!";
        let n = c::write(socket, msg as *const _ as *const c_void, msg.len());
        if n <= 0 {
            println!("last OS error: {:?}", Error::last_os_error());
            c::close(socket);
        }

        let mut buf = [0u8; 64];
        let n = c::read(socket, &mut buf as *mut _ as *mut c_void, buf.len());
        if n <= 0 {
            println!("last OS error: {:?}", Error::last_os_error());
        }

        println!("{:?}", String::from_utf8_lossy(&buf[0..n as usize]));

        c::close(socket);
    }
}