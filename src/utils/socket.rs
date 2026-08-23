// ----------------------------------------------------------------------------
//  socket.rs — Socket2-based TCP error categorisation
// ----------------------------------------------------------------------------
//  Wraps socket2 for direct TCP connect checks and error categorisation.
//  Used as a pre-filter before HTTP requests to detect connectivity issues:
//    Timeout, ConnectionRefused, DNSFailure, ConnectionReset, Other
//
//  --- Developers ---------------------------------------------------------------
//  khaninkali             — разработчик / core engineer (Rust backend, logic)
//  Lyara Koroleva         — дизайнер / blazing fast CLI & interface design
//  HsecDevelopers         — 测试 / テスト / testing & QA (integration, validation)
//  projectk 2091         — HyperSecurityOffensiveLabs lineage
// ----------------------------------------------------------------------------
//
use socket2::{Socket, Domain, Type, Protocol};
use std::io;
use std::net::SocketAddr;
use std::time::Duration;

#[derive(Debug, Clone, PartialEq)]
pub enum SocketError {
    Timeout,
    ConnectionRefused,
    DNSFailure,
    ConnectionReset,
    Other(String),
}

#[derive(Debug, Clone)]
pub struct SocketCheck {
    pub url: String,
    pub host: String,
    pub port: u16,
    pub reachable: bool,
    pub error: Option<SocketError>,
    pub latency_ms: u64,
}

impl SocketCheck {
    pub fn new(host: &str, port: u16) -> Self {
        Self {
            url: format!("{}:{}", host, port),
            host: host.to_string(),
            port,
            reachable: false,
            error: None,
            latency_ms: 0,
        }
    }
}

pub fn check_tcp_connect(host: &str, port: u16, timeout_secs: u64) -> SocketCheck {
    use std::net::ToSocketAddrs;
    let mut result = SocketCheck::new(host, port);

    let addr: SocketAddr = match format!("{}:{}", host, port).to_socket_addrs() {
        Ok(mut addrs) => match addrs.next() {
            Some(a) => a,
            None => {
                result.error = Some(SocketError::DNSFailure);
                return result;
            }
        },
        Err(_) => {
            result.error = Some(SocketError::DNSFailure);
            return result;
        }
    };

    let socket = match Socket::new(Domain::for_address(addr), Type::STREAM, Some(Protocol::TCP)) {
        Ok(s) => s,
        Err(e) => {
            result.error = Some(SocketError::Other(format!("socket creation: {}", e)));
            return result;
        }
    };

    if let Err(e) = socket.set_read_timeout(Some(Duration::from_secs(timeout_secs))) {
        result.error = Some(SocketError::Other(format!("set_read_timeout: {}", e)));
        return result;
    }
    if let Err(e) = socket.set_write_timeout(Some(Duration::from_secs(timeout_secs))) {
        result.error = Some(SocketError::Other(format!("set_write_timeout: {}", e)));
        return result;
    }

    let start = std::time::Instant::now();
    match socket.connect_timeout(&addr.into(), Duration::from_secs(timeout_secs)) {
        Ok(_) => {
            result.reachable = true;
            result.latency_ms = start.elapsed().as_millis() as u64;
        }
        Err(e) => {
            result.latency_ms = start.elapsed().as_millis() as u64;
            result.error = Some(categorise_io_error(&e));
        }
    }

    let _ = socket.shutdown(std::net::Shutdown::Both);
    result
}

fn categorise_io_error(e: &io::Error) -> SocketError {
    match e.kind() {
        io::ErrorKind::TimedOut => SocketError::Timeout,
        io::ErrorKind::ConnectionRefused => SocketError::ConnectionRefused,
        io::ErrorKind::ConnectionReset => SocketError::ConnectionReset,
        io::ErrorKind::ConnectionAborted => SocketError::ConnectionReset,
        io::ErrorKind::AddrNotAvailable => SocketError::DNSFailure,
        io::ErrorKind::InvalidInput => SocketError::DNSFailure,
        _ => {
            let msg = e.to_string().to_lowercase();
            if msg.contains("dns") || msg.contains("resolve") || msg.contains("name") {
                SocketError::DNSFailure
            } else if msg.contains("refused") {
                SocketError::ConnectionRefused
            } else if msg.contains("reset") || msg.contains("broken pipe") {
                SocketError::ConnectionReset
            } else if msg.contains("timeout") || msg.contains("timed out") {
                SocketError::Timeout
            } else {
                SocketError::Other(e.to_string())
            }
        }
    }
}
