use std::net::UdpSocket;

fn main() {
    println!("[LOG] This is a DNS server implementation in Rust!");

    let udp_socket = UdpSocket::bind("0.0.0.0:2053").expect("Failed to bind to address");
    let mut buf = [0; 512];

    loop {
        match udp_socket.recv_from(&mut buf) {
            Ok((size, source)) => {
                println!("Received {} bytes from {}", size, source);
                let response: [u8; 12] = [0x04, 0xD2, 0x80, 0, 0, 0, 0, 0, 0, 0, 0, 0];
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
