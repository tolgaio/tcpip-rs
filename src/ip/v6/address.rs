use std::fmt;

#[derive(Debug)]
pub struct IP6Address {
    parts: Vec<u16>,
}

impl fmt::Display for IP6Address {
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

impl IP6Address {
    pub fn new() -> Self {
        Self { parts: Vec::new() }
    }

    pub fn add_part(&mut self, part: u16) {
        self.parts.push(part);
    }
}
