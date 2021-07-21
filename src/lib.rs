use bitreader::BitReader;
use std::fmt;
use std::vec::Vec;

#[derive(Debug)]
struct IPv6 {
    parts: Vec<u16>,
}

impl fmt::Display for IPv6 {
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Write strictly the first element into the supplied output
        // stream: `f`. Returns `fmt::Result` which indicates whether the
        // operation succeeded or failed. Note that `write!` uses syntax which
        // is very similar to `println!`.
        let mut parts_str: Vec<String> = Vec::new();
        for part in self.parts.iter() {
            parts_str.push(format!("{:x}", part))
        }
        write!(f, "{}", parts_str.join(":"))
    }
}

impl IPv6 {
    fn new() -> Self {
        Self { parts: Vec::new() }
    }

    pub fn add_part(&mut self, part: u16) {
        self.parts.push(part);
    }
}

#[derive(Debug)]
struct IP6 {
    ds: u8,
    ecn: u8,
    flow_label: u32,
    payload_length: u16,
    next_header: u8,
    hop_limit: u8,
    source: Option<IPv6>,
    dest: Option<IPv6>,
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

        write!(
            f,
            "IP6 > SOURCE: {} DESTINATION: {}: <TYPE>, <CODE>, length 8",
            source, dest
        )
    }
}

impl IP6 {
    fn new() -> Self {
        Self {
            ds: 0,
            ecn: 0,
            flow_label: 0,
            payload_length: 0,
            next_header: 0,
            hop_limit: 0,
            source: None,
            dest: None,
            payload: Vec::new(),
        }
    }

    fn parse(&mut self, buf: &[u8]) {
        let mut bit_reader = BitReader::new(buf);
        let _ip_version: u8 = bit_reader.read_u8(4).unwrap();

        self.ds = bit_reader.read_u8(6).unwrap();
        self.ecn = bit_reader.read_u8(2).unwrap();
        self.flow_label = bit_reader.read_u32(20).unwrap();
        self.payload_length = bit_reader.read_u16(16).unwrap();
        self.next_header = bit_reader.read_u8(8).unwrap();
        self.hop_limit = bit_reader.read_u8(8).unwrap();

        // parse ipv6 source ip
        let mut source: IPv6 = IPv6::new();
        for _ip_part in 1..=8 {
            source.add_part(bit_reader.read_u16(16).unwrap());
        }

        // parse ipv6 dest ip
        let mut dest = IPv6::new();
        for _ip_part in 1..=8 {
            dest.add_part(bit_reader.read_u16(16).unwrap());
        }

        self.source = Some(source);
        self.dest = Some(dest);
        self.payload = buf[40..].iter().cloned().collect();
    }
}

pub fn xxx(b: &[u8]) {
    println!("hello => {:?}", b);
}

pub fn parse(buffer: &[u8]) {
    let mut bit_reader = BitReader::new(buffer);
    let ip_version: u8 = bit_reader.read_u8(4).unwrap();

    if ip_version == 6 {
        let mut ip6: IP6 = IP6::new();
        ip6.parse(&buffer);
        println!("{:}", ip6);
        // println!("{:?}", ip6);
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn parse_icmp6() {
        let data: Vec<u8> = vec![
            96, 0, 0, 0, 0, 8, 58, 255, 254, 128, 0, 0, 0, 0, 0, 0, 0, 109, 4, 161, 90, 58, 171,
            52, 255, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 133, 0, 114, 186, 0, 0, 0, 0,
        ];
        let zeros = vec![0; 1500 - data.len()];
        let mut payload: Vec<u8> = Vec::with_capacity(data.len() + zeros.len());
        payload.extend_from_slice(&data);
        payload.extend_from_slice(&zeros);

        super::parse(&payload);
        assert_eq!(2 + 2, 4);
    }
}
