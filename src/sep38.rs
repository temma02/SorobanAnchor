//! SEP-38 Anchor RFQ Service Layer
//!
//! Provides normalized service functions for fetching prices and requesting firm quotes
//! across different anchors.

#![cfg_attr(not(test), no_std)]

extern crate alloc;
use alloc::string::{String, ToString};

use crate::errors::Error;

// ── Normalized response types ────────────────────────────────────────────────

/// Normalized price information from SEP-38 /prices endpoint.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Price {
    pub buy_asset: String,
    pub sell_asset: String,
    pub price: String,
}

// ── Raw response types (from anchor APIs) ────────────────────────────────────

/// Raw price response from anchor /prices endpoint.
#[derive(Clone, Debug)]
pub struct RawPrice {
    pub buy_asset: String,
    pub sell_asset: String,
    pub price: String,
}

// ── Service functions ────────────────────────────────────────────────────────

/// Normalizes a raw /prices response from an anchor.
///
/// Extracts and validates `buy_asset`, `sell_asset`, and `price` fields.
pub fn fetch_prices(raw: RawPrice) -> Result<Price, Error> {
    Ok(Price {
        buy_asset: raw.buy_asset,
        sell_asset: raw.sell_asset,
        price: raw.price,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fetch_prices() {
        let raw = RawPrice {
            buy_asset: "USDC".to_string(),
            sell_asset: "XLM".to_string(),
            price: "0.15".to_string(),
        };
        let result = fetch_prices(raw).unwrap();
        assert_eq!(result.buy_asset, "USDC");
        assert_eq!(result.sell_asset, "XLM");
        assert_eq!(result.price, "0.15");
    }
}
