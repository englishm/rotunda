use rotunda;
use std::io::Error;

#[test]
fn create_nic() -> Result<(),Error>{
    let nic = rotunda::Iface::without_packet_info("tun2", rotunda::Mode::Tun)?;
    println!("{:?}", nic);
    Ok(())
}
