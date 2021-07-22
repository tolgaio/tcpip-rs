# tcpip-rs
TCP/IP implementation in Rust.

### documentations

- [rfc index](https://www.rfc-editor.org/rfc-index.html)
- [rfc official standards tracking](https://www.rfc-editor.org/standards.php?sortkey=Date&sorting=DESC&sort_number=1&showRET=N&showOBS=N#PS)
- [rfc-793-tcp](https://www.rfc-editor.org/rfc/rfc793.html)
- [rfc-1180-tcp-tutorial](https://www.rfc-editor.org/rfc/rfc1180.html)
- [rfc-2398-tcp-testing-tools](https://www.rfc-editor.org/rfc/rfc2398.html)
- [rfc-1256-icmp](https://www.rfc-editor.org/rfc/rfc1256.html)
- [tun-tap](https://www.programmersought.com/article/3923535932/)
- [jumbo frame](https://en.wikipedia.org/wiki/Jumbo_frame)
- [ip6](https://en.wikipedia.org/wiki/IPv6_packet)
- [icmp6](https://en.wikipedia.org/wiki/Internet_Control_Message_Protocol_for_IPv6)
- [rfc-6564 ip6 extension headers](https://datatracker.ietf.org/doc/html/rfc6564)

### C examples

- https://www.cs.swarthmore.edu/~aviv/classes/f12/cs43/labs/lab2.pdf
- http://yuba.stanford.edu/~nickm/papers/ancs48-gibb.pdf
- https://www.sciencedirect.com/topics/engineering/packet-networks

### rust helpers / snippets

- [string-conversion.rs](https://gist.github.com/jimmychu0807/9a89355e642afad0d2aeda52e6ad2424)


- http://www.togglecase.com/convert_to_camel_case
- https://crates.io/crates/num_enum
- https://www.ameyalokare.com/rust/2017/10/23/rust-options.html#fnref:1
- https://stackoverflow.com/questions/28587698/whats-the-difference-between-placing-mut-before-a-variable-name-and-after-the
- [ip command cheat sheet](https://access.redhat.com/sites/default/files/attachments/rh_ip_command_cheatsheet_1214_jcs_print.pdf)
- [tcp header display](https://networklessons.com/cisco/ccie-routing-switching-written/tcp-header)


#### merging vectors

```rust
let mut v = Vec::with_capacity(a.len() + b.len());
v.extend_from_slice(a);
v.extend_from_slice(b);
v
```


#### bit shifting

```rust
fn main() {
    let n = 0b01100000; // let n = 96u8;
    let first_n = 4;
    let shifting = 8 - first_n;
    let shifted = n >> shifting;
    println!("shifting by {}: {}({:b})", shifting, shifted, shifted);
}
```
