use core::fmt;
use std::{alloc::System, write};

#[derive(Hash, PartialEq, Eq, Clone, Copy, Debug)]
pub enum Asset {
    Usdc,
    Eth,
    Sol,
    Usdt,
    Btc,
}

impl fmt::Display for Asset {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Asset::Btc => write!(f, "BTC"),
            Asset::Eth => write!(f, "ETH"),
            Asset::Sol => write!(f, "SOL"),
            Asset::Usdc => write!(f, "USDC"),
            Asset::Usdt => write!(f, "USDT"),
        }
    }
}

impl Asset {
    pub fn asset_decimals(&self) -> u8 {
        match self {
            Asset::Usdc => 6,
            Asset::Usdt => 6,
            Asset::Sol => 9,
            Asset::Btc => 8,
            Asset::Eth => 18,
        }
    }
}
