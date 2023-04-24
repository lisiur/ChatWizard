use std::net::{
    Ipv4Addr, Ipv6Addr, SocketAddrV4, SocketAddrV6, TcpListener, ToSocketAddrs,
};
use std::path::Path;

use crate::result::Result;

pub fn ensure_directory_exists(path: &Path) -> Result<()> {
    if !path.exists() {
        std::fs::create_dir_all(path)?;
    }

    Ok(())
}

pub fn save_file_sync(path: &Path, data: &[u8]) -> Result<()> {
    std::fs::write(path, data)?;

    Ok(())
}

// Try to bind to a socket using TCP
fn test_bind_tcp<A: ToSocketAddrs>(addr: A) -> Option<u16> {
    Some(TcpListener::bind(addr).ok()?.local_addr().ok()?.port())
}

/// Check if a port is free on TCP
pub fn is_free_tcp(port: u16) -> bool {
    let local_ipv4 = SocketAddrV4::new(Ipv4Addr::LOCALHOST, port);
    let local_ipv6 = SocketAddrV6::new(Ipv6Addr::LOCALHOST, port, 0, 0);
    let unspecified_ipv4 = SocketAddrV4::new(Ipv4Addr::UNSPECIFIED, port);
    let unspecified_ipv6 = SocketAddrV6::new(Ipv6Addr::UNSPECIFIED, port, 0, 0);

    test_bind_tcp(local_ipv4).is_some()
        && test_bind_tcp(local_ipv6).is_some()
        && test_bind_tcp(unspecified_ipv6).is_some()
        && test_bind_tcp(unspecified_ipv4).is_some()
}
