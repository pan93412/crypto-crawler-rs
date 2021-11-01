use crypto_markets::{fetch_markets, fetch_symbols, get_market_types, MarketType};

#[macro_use]
mod utils;

const EXCHANGE_NAME: &str = "huobi";

#[test]
fn fetch_all_symbols() {
    gen_all_symbols!();
}

#[test]
fn fetch_spot_symbols() {
    let symbols = fetch_symbols(EXCHANGE_NAME, MarketType::Spot).unwrap();
    assert!(!symbols.is_empty());
    for symbol in symbols.iter() {
        assert_eq!(symbol.to_lowercase(), symbol.to_string());
    }
}

#[test]
fn fetch_inverse_future_symbols() {
    let symbols = fetch_symbols(EXCHANGE_NAME, MarketType::InverseFuture).unwrap();
    assert!(!symbols.is_empty());
    for symbol in symbols.iter() {
        assert!(
            symbol.ends_with("_CW")
                || symbol.ends_with("_NW")
                || symbol.ends_with("_CQ")
                || symbol.ends_with("_NQ")
        );
    }
}

#[test]
fn fetch_inverse_swap_symbols() {
    let symbols = fetch_symbols(EXCHANGE_NAME, MarketType::InverseSwap).unwrap();
    assert!(!symbols.is_empty());
    for symbol in symbols.iter() {
        assert!(symbol.ends_with("-USD"));
    }
}

#[test]
fn fetch_linear_swap_symbols() {
    let symbols = fetch_symbols(EXCHANGE_NAME, MarketType::LinearSwap).unwrap();
    assert!(!symbols.is_empty());
    for symbol in symbols.iter() {
        assert!(symbol.ends_with("-USDT"));
    }
}

#[test]
#[ignore]
fn fetch_option_symbols() {
    let symbols = fetch_symbols(EXCHANGE_NAME, MarketType::EuropeanOption).unwrap();
    assert!(!symbols.is_empty());
    for symbol in symbols.iter() {
        assert!(symbol.contains("-C-") || symbol.contains("-P-"));
    }
}

#[test]
fn fetch_spot_markets() {
    let markets = fetch_markets(EXCHANGE_NAME, MarketType::Spot).unwrap();
    assert!(!markets.is_empty());

    let btc_usdt = markets.iter().find(|m| m.symbol == "btcusdt").unwrap();
    assert_eq!(btc_usdt.precision.tick_size, 0.01);
    assert_eq!(btc_usdt.precision.lot_size, 0.000001);
}
