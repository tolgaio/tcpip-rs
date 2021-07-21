pub mod address;

use super::protocol::{to_ip_protocol, IpProtocol};
use address::IP6Address;
use bitreader::BitReader;
use std::fmt;
use std::vec::Vec;

#[derive(Debug)]
pub struct IP6 {
    ds: u8,
    ecn: u8,
    flow_label: u32,
    payload_length: u16,
    next_header: Option<IpProtocol>,
    hop_limit: u8,
    source: Option<IP6Address>,
    dest: Option<IP6Address>,
    payload: Vec<u8>,
}

impl fmt::Display for IP6 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let source = match &self.source {
            None => "No source".to_string(),
            Some(s) => format!("{}", s),
        };
        let dest = match &self.dest {
            None => "No dest".to_string(),
            Some(d) => format!("{}", d),
        };
        let protocol = match &self.next_header {
            Some(IpProtocol::Tcp) => "tcp".to_string(),
            Some(IpProtocol::Udp) => "udp".to_string(),
            Some(IpProtocol::Icmp) => "icmp".to_string(),
            Some(IpProtocol::Ipv6Icmp) => "ip6-icmp".to_string(),
            Some(_) => "NotImplemented".to_string(),
            None => "UnknownProtocol".to_string(),
        };
        write!(
            f,
            "IP6 > SOURCE: {} DESTINATION: {}: PROTOCOL: {}",
            source, dest, protocol
        )
    }
}

impl IP6 {
    pub fn new() -> Self {
        Self {
            ds: 0,
            ecn: 0,
            flow_label: 0,
            payload_length: 0,
            next_header: None,
            hop_limit: 0,
            source: None,
            dest: None,
            payload: Vec::new(),
        }
    }

    pub fn parse(&mut self, buf: &[u8]) {
        let mut bit_reader = BitReader::new(buf);
        let _ip_version: u8 = bit_reader.read_u8(4).unwrap();

        self.ds = bit_reader.read_u8(6).unwrap();
        self.ecn = bit_reader.read_u8(2).unwrap();
        self.flow_label = bit_reader.read_u32(20).unwrap();
        self.payload_length = bit_reader.read_u16(16).unwrap();
        self.next_header = to_ip_protocol(bit_reader.read_u8(8).unwrap());
        self.hop_limit = bit_reader.read_u8(8).unwrap();

        // parse ip6Address source ip
        let mut source: IP6Address = IP6Address::new();
        for _ip_part in 1..=8 {
            source.add_part(bit_reader.read_u16(16).unwrap());
        }

        // parse ip6Address dest ip
        let mut dest = IP6Address::new();
        for _ip_part in 1..=8 {
            dest.add_part(bit_reader.read_u16(16).unwrap());
        }

        self.source = Some(source);
        self.dest = Some(dest);
        self.payload = buf[40..].iter().cloned().collect();
    }
}
