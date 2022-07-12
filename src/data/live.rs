use crate::data::{
    Feed, MarketGenerator,
    error::DataError,
};
use barter_integration::Subscription;
use barter_data::model::MarketData;
use tokio::sync::{
    mpsc, mpsc::error::TryRecvError,
};
use barter_data::builder::Streams;

/// Todo:
pub struct MarketFeed {
    pub market_rx: mpsc::UnboundedReceiver<MarketData>,
}

impl MarketGenerator for MarketFeed {
    fn generate(&mut self) -> Feed<MarketData> {
        loop {
            match self.market_rx.try_recv() {
                Ok(market) => {
                    break Feed::Next(market)
                },
                Err(TryRecvError::Empty) => {
                    continue
                },
                Err(TryRecvError::Disconnected) => {
                    break Feed::Unhealthy
                }
            }
        }
    }
}

impl MarketFeed {
    pub async fn init(subscriptions: &[Subscription]) -> Result<Self, DataError> {
        let streams = Streams::builder()
            .subscribe_new(subscriptions)
            .init()
            .await?;

        Ok(Self { market_rx: streams.join().await })
    }
}