pub enum IPProto {
    ICMP = 1,
    TCP = 6,
    UDP = 17,
}

pub enum IPFlags {
    reserved = 1,
    must_fragment = 2,
    do_not_fragment = 4,
}

pub struct IPv4Header {
    version_ihl: u8,
    tos_ecn: u8,
    packet_length: u16,
    identification: u16,
    fragment_offset_flags: u16,
    time_to_live: u8, 
    protocol: IPProto,
    checksum: u16,
    source_ip: u32,
    dest_ip: u32,
}

impl IPv4Header {
    pub fn new() -> IPv4Header {
        IPv4Header {
            version_ihl: 0,
            tos_ecn: 0,
            packet_length: 0,
            identification: 0,
            fragment_offset_flags: 0,
            time_to_live: 0,
            protocol: IPProto::ICMP,
            checksum: 0,
            source_ip: 0,
            dest_ip: 0,
        }
    }

    pub fn get_version_ihl(&self) -> u8 {
        self.version_ihl >> 4
    }

    pub fn get_header_length(&self) -> u8 {
        (self.version_ihl & 0x0f) * 4
    }

    pub fn get_length(&self) -> u16 {
        u16::from_le(self.packet_length)
    }

    pub fn get_id(&self) -> u16 {
        u16::from_le(self.identification)
    }

    pub fn get_flags(&self) -> u16 {
        ((self.fragment_offset_flags & 0x0e) >> 4)
    }

    pub fn get_fragment_offset(&self) -> u16 {
        u16::from_le(self.fragment_offset_flags) << 3
    }

    
}

