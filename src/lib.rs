use serde::{Serialize, Deserialize};

pub mod api;

// Client vars
pub static SUFFIX: &str = ".wannaplay";

// Server Vars
pub const PORT: u16 = 9999;
pub static PATH_DB: &str = "./sled.db";

pub trait ExpectLog {
    type Ret;
    fn expect_error(self, message: &str) -> Self::Ret;
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct Message {
    pub mac_addr: [u8; 6],
    pub key: String,
    pub completed: bool,
}