pub mod protocol;
pub mod v6;

#[derive(Debug, PartialEq)]
pub enum IP {
    V6(v6::IP6),
}
