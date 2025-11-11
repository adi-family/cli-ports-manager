use std::net::TcpListener;

pub fn find_available_port(exclude_ports: &[u16]) -> Option<u16> {
    find_available_port_in_range(8000, 65535, exclude_ports)
}

pub fn find_available_port_in_range(start: u16, end: u16, exclude_ports: &[u16]) -> Option<u16> {
    for port in start..=end {
        if !exclude_ports.contains(&port) && is_port_available(port) {
            return Some(port);
        }
    }
    None
}

fn is_port_available(port: u16) -> bool {
    TcpListener::bind(format!("127.0.0.1:{}", port)).is_ok()
}
