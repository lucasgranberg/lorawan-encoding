pub mod mac_command;
pub mod phy_payload;
pub mod types;

#[derive(Debug)]
pub enum Error {
    Size,
}
