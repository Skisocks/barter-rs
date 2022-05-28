use crate::event::Event;

use crate::data::{MarketEvent, MarketMeta};
use crate::strategy::{Decision, Signal, SignalEvent, SignalForceExit};
use crate::execution::{FillEvent, Trade};
use crate::portfolio::{
    error::PortfolioError,
    order::{Order, Request},
    position::PositionUpdate
};
use barter_data::model::MarketData;
use barter_integration::Instrument;
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use uuid::Uuid;

/// Logic for [`OrderEvent`] quantity allocation.
pub mod allocator;

/// Barter portfolio module specific errors.
pub mod error;

/// Core Portfolio logic containing an implementation of [`MarketUpdater`],
/// [`OrderGenerator`] and [`FillUpdater`]. Utilises the risk and allocator logic to optimise
/// [`OrderEvent`] generation.
pub mod portfolio;

/// Data structures encapsulating the state of a trading [`Position`], as well as the logic for
/// entering, updating and exiting them.
pub mod position;

/// Repositories for persisting Portfolio state.
pub mod repository;

/// Logic for evaluating the risk associated with a proposed [`OrderEvent`].
pub mod risk;

/// Todo:
pub mod order;

/// Updates the Portfolio from an input [`MarketEvent`].
pub trait MarketUpdater {
    /// Determines if the Portfolio has an open Position relating to the input [`MarketEvent`]. If
    /// so it updates it using the market data, and returns a [`PositionUpdate`] detailing the
    /// changes.
    fn update_from_market(
        &mut self,
        market: &MarketData,
    ) -> Result<Option<PositionUpdate>, PortfolioError>;
}

/// May generate an [`OrderEvent`] from an input advisory [`SignalEvent`].
pub trait OrderGenerator {
    /// May generate an [`OrderEvent`] after analysing an input advisory [`SignalEvent`].
    fn generate_order(
        &mut self,
        signal: &Signal,
    ) -> Result<Option<Order<Request>>, PortfolioError>;

    /// Generates an exit [`OrderEvent`] if there is an open [`Position`] associated with the
    /// input [`SignalForceExit`]'s [`PositionId`].
    fn generate_exit_order(
        &mut self,
        signal: SignalForceExit,
    ) -> Result<Option<Order<Request>>, PortfolioError>;
}

/// Todo:
/// Updates the Portfolio from an input [`FillEvent`].
pub trait FillUpdater {
    /// Updates the Portfolio state using the input [`FillEvent`]. The [`FillEvent`] triggers a
    /// Position entry or exit, and the Portfolio updates key fields such as current_cash and
    /// current_value accordingly.
    fn update_from_trade(
        &mut self,
        trade: &Trade,
    ) -> Result<Vec<AccountEvent>, PortfolioError>;
}

/// Todo:
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Deserialize, Serialize)]
pub struct Balance {
    pub instrument: Instrument,
    pub total: f64,
    pub free: f64,
    pub used: f64,
}