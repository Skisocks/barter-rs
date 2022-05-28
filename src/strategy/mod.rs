/// Barter strategy module specific errors.
pub mod error;

///
pub mod strategy;

use crate::Market;
use crate::data::{MarketEvent, MarketMeta};
use crate::strategy::error::StrategyError;
use std::collections::HashMap;
use chrono::{DateTime, Utc};
use uuid::Uuid;
use serde::{Deserialize, Serialize};
use barter_data::model::MarketData;
use barter_integration::{Exchange, Instrument};

/// May generate an advisory [`Signal`] as a result of analysing input [`MarketData`].
pub trait SignalGenerator {
    /// Optionally return a [`Signal`] given input [`MarketData`].
    fn generate_signal(&mut self, market: &MarketData) -> Option<Signal>;
}

/// Strength of an advisory signal decision produced by the strategy.
pub struct SignalStrength(pub f32);

/// Todo:
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Deserialize, Serialize)]
pub struct Signal {
    pub timestamp: DateTime<Utc>,
    pub exchange: &'static str,
    pub instrument: Instrument,
    pub signals: HashMap<Decision, SignalStrength>,
    /// Metadata propagated from source MarketEvent
    pub market_meta: MarketMeta,
}

/// Describes the type of advisory signal the strategy is endorsing.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Deserialize, Serialize)]
pub enum Decision {
    Long,
    CloseLong,
    Short,
    CloseShort,
}

impl Decision {
    /// Determines if a [`Decision`] is Long.
    pub fn is_long(&self) -> bool {
        matches!(self, Decision::Long)
    }

    /// Determines if a [`Decision`] is Short.
    pub fn is_short(&self) -> bool {
        matches!(self, Decision::Short)
    }

    /// Determines if a [`Decision`] is an entry (long or short).
    pub fn is_entry(&self) -> bool {
        matches!(self, Decision::Short | Decision::Long)
    }

    /// Determines if a [`Decision`] is an exit (close_long or close_short).
    pub fn is_exit(&self) -> bool {
        matches!(self, Decision::CloseLong | Decision::CloseShort)
    }
}

/// Force exit Signal produced after an [`Engine`] receives a [`Command::ExitPosition`](Command)
/// from an external source.
#[derive(Clone, PartialEq, PartialOrd, Debug, Deserialize, Serialize)]
pub struct SignalForceExit {
    pub timestamp: DateTime<Utc>,
    pub exchange: Exchange,
    pub instrument: Instrument,
}

impl SignalForceExit {
    pub const FORCED_EXIT_SIGNAL: &'static str = "SignalForcedExit";

    /// Constructs a new [`Self`] using the configuration provided.
    pub fn new(exchange: Exchange, instrument: Instrument) -> Self {
        Self {
            timestamp: Utc::now(),
            exchange,
            instrument,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_return_decision_is_long() {
        let decision = Decision::Long;
        assert_eq!(decision.is_long(), true)
    }

    #[test]
    fn should_return_decision_is_not_long() {
        let decision = Decision::Short;
        assert_eq!(decision.is_long(), false)
    }

    #[test]
    fn should_return_decision_is_short() {
        let decision = Decision::Short;
        assert_eq!(decision.is_short(), true)
    }

    #[test]
    fn should_return_decision_is_not_short() {
        let decision = Decision::Long;
        assert_eq!(decision.is_short(), false)
    }

    #[test]
    fn should_return_decision_is_entry() {
        let decision = Decision::Long;
        assert_eq!(decision.is_entry(), true)
    }

    #[test]
    fn should_return_decision_is_not_entry() {
        let decision = Decision::CloseLong;
        assert_eq!(decision.is_entry(), false)
    }

    #[test]
    fn should_return_decision_is_exit() {
        let decision = Decision::CloseShort;
        assert_eq!(decision.is_exit(), true)
    }

    #[test]
    fn should_return_decision_is_not_exit() {
        let decision = Decision::Long;
        assert_eq!(decision.is_exit(), false)
    }
}