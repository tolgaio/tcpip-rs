use super::types;
use bitreader::BitReader;
use std::fmt;

#[derive(Debug)]
pub struct ICMP6 {
    pub typ: Option<types::ICMPType>,
    pub code: u8,
    pub checksum: u16,
    pub payload: Vec<u8>,
}

impl ICMP6 {
    pub fn new(buf: &[u8]) -> Self {
        let mut icmp = Self {
            typ: None,
            code: 0,
            checksum: 0,
            payload: Vec::new(),
        };
        icmp.parse(buf);
        icmp
    }

    fn parse(&mut self, buf: &[u8]) {
        let mut bit_reader = BitReader::new(buf);
        self.typ = types::to_icmp_type(bit_reader.read_u8(8).unwrap());
        self.code = bit_reader.read_u8(8).unwrap();
        self.checksum = bit_reader.read_u16(16).unwrap();
        self.payload = buf[8..].iter().cloned().collect();
    }
}

impl fmt::Display for ICMP6 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let typ = match &self.typ {
            Some(types::ICMPType::RouterSolicitation) => "router solicitation".to_string(),
            Some(_) => "NotImplemented".to_string(),
            None => "UnknownType".to_string(),
        };
        write!(f, "ICMP > TYPE: {}", typ)
    }
}

#[cfg(test)]
mod tests {
    use super::types;
    #[test]
    fn parse_icmp6() {
        let data: Vec<u8> = vec![133, 0, 114, 186, 0, 0, 0, 0];
        let icmp6 = super::ICMP6::new(&data);
        println!("{}", icmp6);
        assert_eq!(icmp6.typ, Some(types::ICMPType::RouterSolicitation));
    }
}
