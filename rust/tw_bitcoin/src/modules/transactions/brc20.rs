use super::ordinals::OrdinalsInscription;
use crate::{Error, Result};
use bitcoin::PublicKey;
use serde::{Serialize};
use tw_proto::BitcoinV2::Proto;

#[derive(Debug, Clone, Serialize)]
pub struct BRC20Payload<T> {
    #[serde(rename = "p")]
    protocol: String,
    #[serde(rename = "op")]
    operation: String,
    #[serde(flatten)]
    inner: T,
}

impl<T> BRC20Payload<T> {
    const PROTOCOL_ID: &str = "brc-20";
    const MIME: &[u8] = b"text/plain;charset=utf-8";
}


#[derive(Serialize)]
pub struct DeployPayload {
    pub tick: Brc20Ticker,
    pub max: String,
    pub lim: String,
    pub dec: String,
}

/// The structure is the same as `TransferPayload`, but we'll keep it separate
/// for clarity.
#[derive(Serialize)]
pub struct MintPayload {
    pub tick: Brc20Ticker,
    pub amt: String,
}

#[derive(Serialize)]
pub struct TransferPayload {
    pub tick: Brc20Ticker,
    pub amt: String,
}

pub type BRC20DeployPayload = BRC20Payload<DeployPayload>;
pub type BRC20MintPayload = BRC20Payload<MintPayload>;
pub type BRC20TransferPayload = BRC20Payload<TransferPayload>;

impl BRC20DeployPayload {
    const OPERATION: &str = "deploy";

    pub fn new(ticker: Brc20Ticker, max: u64, limit: u64, decimals: u64) -> Self {
        BRC20Payload {
            protocol: Self::PROTOCOL_ID.to_string(),
            operation: Self::OPERATION.to_string(),
            inner: DeployPayload {
                tick: ticker,
                max: max.to_string(),
                lim: limit.to_string(),
                dec: decimals.to_string(),
            },
        }
    }
}

impl BRC20MintPayload {
    const OPERATION: &str = "mint";

    pub fn new(ticker: Brc20Ticker, amount: u64) -> Self {
        BRC20Payload {
            protocol: Self::PROTOCOL_ID.to_string(),
            operation: Self::OPERATION.to_string(),
            inner: MintPayload {
                tick: ticker,
                amt: amount.to_string(),
            },
        }
    }
}

impl BRC20TransferPayload {
    const OPERATION: &str = "transfer";

    fn new(ticker: Brc20Ticker, value: u64) -> Self {
        BRC20Payload {
            protocol: Self::PROTOCOL_ID.to_string(),
            operation: Self::OPERATION.to_string(),
            inner: TransferPayload { 
                tick: ticker, 
                amt: value.to_string(), 
            },
        }
    }
}



#[derive(Debug, Clone, Serialize)]
pub struct Brc20Ticker(String);

impl Brc20Ticker {
    pub fn new(string: String) -> Result<Self> {
        // Brc20Ticker must be a 4-letter identifier.
        if string.len() != 4 {
            return Err(Error::from(Proto::Error::Error_invalid_brc20_ticker));
        }

        Ok(Brc20Ticker(string))
    }
}

pub struct BRC20DeployInscription(OrdinalsInscription);
impl BRC20DeployInscription {
    pub fn new(
        recipient: PublicKey,
        ticker: Brc20Ticker,
        max: u64,
        limit: u64,
        decimals: u64,
    ) -> Result<BRC20DeployInscription> {
        let data = BRC20DeployPayload::new(ticker, max, limit, decimals);
        let inscription = OrdinalsInscription::new(
            BRC20DeployPayload::MIME,
            &serde_json::to_vec(&data).expect("badly constructed BRC20 payload"),
            recipient,
        )?;

        Ok(BRC20DeployInscription(inscription))
    }
    pub fn inscription(&self) -> &OrdinalsInscription {
        &self.0
    }
}


pub struct BRC20TransferInscription(OrdinalsInscription);

impl BRC20TransferInscription {
    pub fn new(
        recipient: PublicKey,
        ticker: Brc20Ticker,
        value: u64,
    ) -> Result<BRC20TransferInscription> {
        let data: BRC20TransferPayload = BRC20TransferPayload::new(ticker, value);

        let inscription = OrdinalsInscription::new(
            BRC20TransferPayload::MIME,
            &serde_json::to_vec(&data).expect("badly constructed BRC20 payload"),
            recipient,
        )?;

        Ok(BRC20TransferInscription(inscription))
    }
    pub fn inscription(&self) -> &OrdinalsInscription {
        &self.0
    }
}

pub struct BRC20MintInscription(OrdinalsInscription);

impl BRC20MintInscription {
    pub fn new(
        recipient: PublicKey,
        ticker: Brc20Ticker,
        amount: u64,
    ) -> Result<BRC20MintInscription> {
        let data = BRC20MintPayload::new(ticker, amount);
        
        let inscription = OrdinalsInscription::new(
            BRC20MintPayload::MIME,
            &serde_json::to_vec(&data).expect("badly constructed BRC20 payload"),
            recipient,
        )?;
        Ok(BRC20MintInscription(inscription))
    }
    
    pub fn inscription(&self) -> &OrdinalsInscription {
        &self.0
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn brc20_ticker_validity() {
        // Must be four characters.
        let ticker = Brc20Ticker::new("invalid".to_string());
        assert!(ticker.is_err());

        let ticker = Brc20Ticker::new("asdf".to_string());
        assert!(ticker.is_ok());

        // Cover clone implemenation.
        let ticker = ticker.unwrap();

        let _cloned = ticker.clone();
        let _ticker = ticker;
    }
}
