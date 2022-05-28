use chrono::{DateTime, Utc};
use uuid::Uuid;

use crate::data::MarketMeta;
use crate::portfolio::OrderEvent;
use crate::execution::error::ExecutionError;
use serde::{Deserialize, Serialize};
use barter_data::model::Side;
use barter_integration::Instrument;
use crate::strategy::Decision;

/// Barter execution module specific errors.
pub mod error;

/// Handlers for simulated and live [`OrderEvent`] execution.
pub mod simulated;

/// Generates a result [`FillEvent`] by executing an [`OrderEvent`].
pub trait ExecutionClient {
    /// Return a [`FillEvent`] from executing the input [`OrderEvent`].
    fn generate_fill(&self, order: &OrderEvent) -> Result<FillEvent, ExecutionError>;
}

/// Todo:
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Deserialize, Serialize)]
pub struct TradeId(&'static str);

/// Todo:
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Deserialize, Serialize)]
pub struct Trade {
    pub id: TradeId,
    pub order_id: OrderId,
    pub client_order_id: ClientOrderId,
    pub instrument: Instrument,
    pub side: Side,
    pub price: f64,
    pub quantity: f64,
    pub fees: f64,
}