struct TCPHeader {
    source_port: u16,
    dest_port: u16,
    sequence_number: u32,
    ack_number: u32,
    data_offset_ns: u8,
    flags: u8,
    window_size: u16,
    checksum: u16,
    urgent: u16,
}

impl TCPHeader {
    pub fn new() -> TCPHeader {
        TCPHeader {
            source_port: 0,
            dest_port: 0,
            sequence_number: 0,
            ack_number: 0,
            data_offset_ns: 0,
            flags: 0,
            window_size: 0,
            checksum: 0,
            urgent: 0,
        }
    }

    pub fn get_source_port(&self) -> u16 {
        u16::from_le(self.source_port)
    }

    pub fn get_destination_port(&self) -> u16 {
        u16::from_le(self.dest_port)
    }

    pub fn get_sequence_number(&self) -> u32 {
        u32::from_le(self.sequence_number)
    }

    pub fn get_data_offset(&self) -> u8 {
        (self.data_offset_ns >> 4) * 4
    }

    pub fn get_ack_number(&self) -> u32 {
        u32::from_le(self.ack_number)
    }

    pub fn get_window_size(&self) -> u16 {
        u16::from_le(self.window_size)
    }

    pub fn get_checksum(&self) -> u16 {
        u16::from_le(self.checksum)
    }

    pub fn get_urgent(&self) -> u16 {
        u16::from_le(self.urgent)
    }
}