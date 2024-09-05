use models::{ResponseEnvelop, SeasonCompetitorsBody, SeasonalCompetitorStatistics};

pub mod models;
pub mod sport_radar;

// Interface for data fetching.
// Such a simple program doesn't really need one, so it's mostly to give an idea how to extend it in the future.
#[async_trait::async_trait]
pub trait DataProvider {
    async fn get_season_competitors(
        &self,
        season_id: &str,
    ) -> anyhow::Result<ResponseEnvelop<SeasonCompetitorsBody>>;

    async fn get_season_competitor_statictic(
        &self,
        season_id: &str,
        competitor_id: &str,
    ) -> anyhow::Result<ResponseEnvelop<SeasonalCompetitorStatistics>>;
}
