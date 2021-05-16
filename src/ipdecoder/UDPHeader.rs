
struct UDPHeader {
    source_port: u16,
    destination_port: u16,
    length: u16,
    checksum: u16,
}

impl UDPHeader {
    pub fn new() -> UDPHeader {
        UdpHeader {
            source_port: 0,
            destination_port: 0,
            length: 0,
            checksum: 0,
        }
    }
}