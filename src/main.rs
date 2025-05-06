use std::net::UdpSocket;

const DNS_BUFFER_SIZE: usize = 512;
const DNS_HEADER_SIZE: usize = 12;

trait ToBytes {
    fn write_to(&self, buffer: &mut [u8], offset: &mut usize) -> Result<(), &str>;
}
struct DnsHeader {
    id: u16,
    flags: u16,
    qdcount: u16,
    ancount: u16,
    nscount: u16,
    arcount: u16,
}

impl DnsHeader {
    fn new(id: u16, flags: u16, qdcount: u16, ancount: u16, nscount: u16, arcount: u16) -> Self {
        Self {
            id,
            flags,
            qdcount,
            ancount,
            nscount,
            arcount,
        }
    }
}

fn main() {
    println!("[LOG] This is a DNS server implementation in Rust!");

    let udp_socket = UdpSocket::bind("127.0.0.1:2053").expect("Failed to bind to address");
    let mut buf = [0u8; DNS_BUFFER_SIZE];

    loop {
        match udp_socket.recv_from(&mut buf) {
            Ok((size, source)) => {
                println!("Received {} bytes from {}", size, source);
                let response: [u8; DNS_HEADER_SIZE] = [0x04, 0xD2, 0x80, 0, 0, 0, 0, 0, 0, 0, 0, 0];
                udp_socket
                    .send_to(&response, source)
                    .expect("Failed to send response");
            }
            Err(e) => {
                eprintln!("Error receiving data: {}", e);
                break;
            }
        }
    }
}

fn read_be_16(buffer: &[u8], offset: usize) -> Result<u16, &str> {
    if offset + 1 >= buffer.len() {
        Err("Buffer overflow while reading u16!")
    } else {
        let num = (buffer[offset] as u16) << 8 | buffer[offset + 1] as u16;
        Ok(num)
    }
}

fn read_header(buffer: &[u8]) -> Result<DnsHeader, &str> {
    Ok(DnsHeader::new(
        u16::from_be_bytes([buffer[0], buffer[1]]),
        read_be_16(buffer, 2)?,
        read_be_16(buffer, 4)?,
        read_be_16(buffer, 6)?,
        read_be_16(buffer, 8)?,
        read_be_16(buffer, 10)?,
    ))
}

fn process_query(buffer: &[u8; DNS_BUFFER_SIZE]) {
    let dns_header = read_header(&buffer[0..DNS_HEADER_SIZE]);
}
