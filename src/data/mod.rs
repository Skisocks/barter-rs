use crate::data::error::DataError;
use barter_data::model::MarketData;
use chrono::{DateTime, Utc};
use uuid::Uuid;
use serde::{Deserialize, Serialize};

/// Barter data module specific errors.
pub mod error;

/// Live [`MarketData`] feed for dry-trading & live-trading.
pub mod live;

/// Historical [`MarketData`] feed for backtesting.
pub mod historical;

/// Generates the latest [`MarketData`]. Acts as the system heartbeat.
pub trait MarketGenerator {
    /// Return the latest [`MarketData`].
    fn generate(&mut self) -> Feed<MarketData>;
}

/// Todo: Where to put this? Integration? Arguably this could all live in barter-data?
pub enum Feed<Event> {
    Next(Event),
    Finished,
    Unhealthy
}

/// Metadata detailing the [`Candle`] close price & it's associated timestamp. Used to propagate key
/// market information in downstream Events.
#[derive(Copy, Clone, PartialEq, PartialOrd, Debug, Deserialize, Serialize)]
pub struct MarketMeta {
    /// [`Candle`] close value from the source [`MarketEvent`].
    pub close: f64,
    /// [`Candle`] timestamp from the source [`MarketEvent`].
    pub timestamp: DateTime<Utc>,
}