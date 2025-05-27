use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Serialize, Clone, Default)]
pub struct Tick {
    pub bid: String,
    pub ask: String,
}

pub trait Exchange: Send + Sync {
    fn name(&self) -> &'static str;
    fn url(&self) -> &'static str;
    fn parse(&self, body: &str) -> Result<Tick, String>;
}

#[derive(Deserialize)]
struct BinanceDepth {
    bids: Vec<[String; 2]>,
    asks: Vec<[String; 2]>,
}

pub struct Binance;
impl Exchange for Binance {
    fn name(&self) -> &'static str { "binance" }
    fn url(&self) -> &'static str {
        "https://api.binance.us/api/v3/depth?symbol=BTCUSDT&limit=1"
    }
    fn parse(&self, body: &str) -> Result<Tick, String> {
        let d: BinanceDepth = serde_json::from_str(body).map_err(|e| e.to_string())?;
        Ok(Tick { bid: d.bids[0][0].clone(), ask: d.asks[0][0].clone() })
    }
}

#[derive(Deserialize)]
struct MEXCDepth {
    bids: Vec<[String; 2]>,
    asks: Vec<[String; 2]>,
}

pub struct MEXC;
impl Exchange for MEXC {
    fn name(&self) -> &'static str { "mexc" }
    fn url(&self) -> &'static str {
        "https://api.mexc.com/api/v3/depth?symbol=BTCUSDT&limit=1"
    }
    fn parse(&self, body: &str) -> Result<Tick, String> {
        let d: MEXCDepth = serde_json::from_str(body).map_err(|e| e.to_string())?;
        Ok(Tick { bid: d.bids[0][0].clone(), ask: d.asks[0][0].clone() })
    }
}

#[derive(Deserialize)]
struct LbankData {
    bids: Vec<[String; 2]>,
    asks: Vec<[String; 2]>,
}

#[derive(Deserialize)]
struct LbankDepth {
    data: LbankData,
}

pub struct Lbank;
impl Exchange for Lbank {
    fn name(&self) -> &'static str { "lbank" }
    fn url(&self) -> &'static str {
        "https://api.lbank.info/v2/depth.do?symbol=btc_usdt&size=1"
    }
    fn parse(&self, body: &str) -> Result<Tick, String> {
        let d: LbankDepth = serde_json::from_str(body).map_err(|e| e.to_string())?;
        Ok(Tick { bid: d.data.bids[0][0].clone(), ask: d.data.asks[0][0].clone() })
    }
}

#[derive(Deserialize, Debug)]
struct CoinbaseDepth {
    bids: Vec<[Value; 3]>,
    asks: Vec<[Value; 3]>,
}

pub struct Coinbase;
impl Exchange for Coinbase {
    fn name(&self) -> &'static str { "coinbase" }
    fn url(&self) -> &'static str {
        "https://api.exchange.coinbase.com/products/BTC-USD/book?level=1"
    }
    fn parse(&self, body: &str) -> Result<Tick, String> {
        let d: CoinbaseDepth = serde_json::from_str(body).map_err(|e| e.to_string())?;
        Ok(Tick {
            bid: d.bids[0][0].as_str().unwrap_or_default().to_string(),
            ask: d.asks[0][0].as_str().unwrap_or_default().to_string(),
        })
    }
}
