/** Implements the libmachina network library for use in rust **/
/** Specifically implements IPDecoder.cs from https://github.com/ravahn/machina **/

pub use self::ipv4header::IPv4Header;
mod ipv4header;

use std::convert::TryInto;
use std::mem;

struct IPDecoder {
	proto: u8,
	source_ip: u32,
	dest_ip: u32,
	fragments: Vec<Vec<u8>>,
	last_fragment_timestamp: u64,
	last_ip_fragment_timestamp: u64,
}

impl IPDecoder {
	pub fn new(proto: u8, source_ip: u32, dest_ip: u32) -> IPDecoder {
		 let decoder = IPDecoder {
		 	proto: proto,
		 	source_ip: source_ip,
		 	dest_ip: dest_ip,
		 	fragments: Vec::new(),
		 	last_fragment_timestamp: 0,
		 	last_ip_fragment_timestamp: 0,
		 };
		 decoder
	}
	pub fn filter_and_store(buf: Vec<u8>, size: u32) {
		let mut offset = 0;

		if(buf.len() == 0)
		{
			return;
		}
		if buf.len() < size.try_into().unwrap() {
			println!("Err: packet size size is too small");
			return;
		}
		while offset < (size - 20) { // 20 represents the length of the IP Header

		}
	}
}