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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_available_port_no_exclusions() {
        let result = find_available_port(&[]);
        assert!(result.is_some());
        let port = result.unwrap();
        assert!(port >= 8000);
    }

    #[test]
    fn test_find_available_port_with_exclusions() {
        let exclude = vec![8000, 8001, 8002];
        let result = find_available_port(&exclude);
        assert!(result.is_some());
        let port = result.unwrap();
        assert!(!exclude.contains(&port));
        assert!(port >= 8000);
    }

    #[test]
    fn test_find_available_port_in_range_small_range() {
        let result = find_available_port_in_range(9000, 9010, &[]);
        assert!(result.is_some());
        let port = result.unwrap();
        assert!(port >= 9000);
        assert!(port <= 9010);
    }

    #[test]
    fn test_find_available_port_in_range_with_exclusions() {
        let exclude = vec![9000, 9001, 9002];
        let result = find_available_port_in_range(9000, 9010, &exclude);
        assert!(result.is_some());
        let port = result.unwrap();
        assert!(!exclude.contains(&port));
        assert!(port >= 9003);
        assert!(port <= 9010);
    }

    #[test]
    fn test_find_available_port_in_range_single_port() {
        let result = find_available_port_in_range(9500, 9500, &[]);
        // May or may not find a port depending on system state
        if let Some(port) = result {
            assert_eq!(port, 9500);
        }
    }

    #[test]
    fn test_find_available_port_in_range_all_excluded() {
        let exclude = vec![9100, 9101, 9102];
        let result = find_available_port_in_range(9100, 9102, &exclude);
        assert!(result.is_none());
    }

    #[test]
    fn test_is_port_available() {
        // Find an available port in a high range to avoid conflicts
        let port = find_available_port_in_range(60000, 65535, &[]).unwrap();

        // Bind to it to make it unavailable
        let _listener = TcpListener::bind(format!("127.0.0.1:{}", port)).unwrap();
        // Now it should not be available
        assert!(!is_port_available(port));

        // After dropping the listener, it should become available again
        drop(_listener);
        // Small delay to ensure port is released
        std::thread::sleep(std::time::Duration::from_millis(10));
        assert!(is_port_available(port));
    }

    #[test]
    fn test_multiple_sequential_ports() {
        let result1 = find_available_port(&[]);
        assert!(result1.is_some());
        let port1 = result1.unwrap();

        let result2 = find_available_port(&[port1]);
        assert!(result2.is_some());
        let port2 = result2.unwrap();

        assert_ne!(port1, port2);
    }

    #[test]
    fn test_find_available_port_excludes_correctly() {
        // Exclude a large range and ensure we get a port outside of it
        let exclude: Vec<u16> = (8000..8100).collect();
        let result = find_available_port(&exclude);
        assert!(result.is_some());
        let port = result.unwrap();
        assert!(port >= 8100 || port < 8000);
    }

    #[test]
    fn test_find_available_port_empty_exclude_list() {
        let result = find_available_port(&vec![]);
        assert!(result.is_some());
        assert!(result.unwrap() >= 8000);
    }

    #[test]
    fn test_find_available_port_in_range_boundary_start() {
        let result = find_available_port_in_range(8000, 8100, &[]);
        assert!(result.is_some());
        let port = result.unwrap();
        assert!(port >= 8000);
        assert!(port <= 8100);
    }

    #[test]
    fn test_find_available_port_in_range_boundary_end() {
        let result = find_available_port_in_range(65530, 65535, &[]);
        assert!(result.is_some());
        let port = result.unwrap();
        assert!(port >= 65530);
    }

    #[test]
    fn test_port_exclusion_order_independent() {
        let exclude1 = vec![8000, 8001, 8002];
        let exclude2 = vec![8002, 8000, 8001];

        let result1 = find_available_port_in_range(8000, 8010, &exclude1);
        let result2 = find_available_port_in_range(8000, 8010, &exclude2);

        assert!(result1.is_some());
        assert!(result2.is_some());
        // Both should find the same first available port
        assert_eq!(result1.unwrap(), result2.unwrap());
    }
}
