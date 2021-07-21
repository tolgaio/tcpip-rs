use bitreader::BitReader;

mod icmp;
mod ip;

use ip::v6::IP6;

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
