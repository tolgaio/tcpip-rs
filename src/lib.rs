use bitreader::BitReader;

mod error;
mod icmp;
mod ip;

use ip::v6::IP6;

pub fn parse(buffer: &[u8]) -> Result<(), error::IPError> {
    if buffer.len() < 21 {
        // 20 bytes for the header, and 1 byte of data
        return Err(error::IPError::NotEnoughBytes(buffer.len() as u8));
    }
    if buffer.len() > 65_535 {
        return Err(error::IPError::TooManyBytes(buffer.len() as u32));
    }

    let mut bit_reader = BitReader::new(buffer);
    let ip_version: u8 = bit_reader.read_u8(4).unwrap();

    match ip_version {
        4 => {
            println!("ipv4 is detected");
            Ok(())
        }
        6 => {
            let mut ip6: IP6 = IP6::new();
            ip6.parse(&buffer);
            Ok(())
        }
        v => Err(error::IPError::InvalidVersion(v)),
    }
}

#[cfg(test)]
mod tests {
    use super::error;
    use super::parse;

    #[test]
    fn parse_invalid_packet_not_enough_bytes() {
        assert_eq!(Err(error::IPError::NotEnoughBytes(1)), parse(&vec![0]));
    }

    #[test]
    fn parse_invalid_packet_too_many_bytes() {
        assert_eq!(
            Err(error::IPError::TooManyBytes(65_535 + 1)),
            parse(&vec![0; 65_535 + 1])
        );
    }

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
        assert_eq!(Ok(()), parse(&payload));
    }

    #[test]
    fn parse_invalid_version() {
        let invalid_version: u8 = 0b0111;
        let data: Vec<u8> = vec![invalid_version << 4];
        let zeros = vec![0; 22 - data.len()];
        let mut payload: Vec<u8> = Vec::with_capacity(data.len() + zeros.len());
        payload.extend_from_slice(&data);
        payload.extend_from_slice(&zeros);
        assert_eq!(
            Err(error::IPError::InvalidVersion(invalid_version)),
            parse(&payload)
        );
    }
}
