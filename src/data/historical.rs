use crate::data::{Feed, MarketGenerator};
use barter_data::model::MarketData;

/// Todo:
pub struct MarketFeed<I>
where
    I: Iterator<Item = MarketData>,
{
    pub market_iterator: I
}

impl<I> MarketGenerator for MarketFeed<I>
where
    I: Iterator<Item = MarketData>
{
    fn generate(&mut self) -> Feed<MarketData> {
        self.market_iterator
            .next()
            .map_or(Feed::Finished, |data| Feed::Next(data))
    }
}

impl<I> MarketFeed<I>
where
    I: Iterator<Item = MarketData>
{
    pub fn new(market_iterator: I) -> Self {
        Self { market_iterator }
    }
}

