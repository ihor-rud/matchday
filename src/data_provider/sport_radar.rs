use serde::de::DeserializeOwned;

use super::models::{ResponseEnvelop, SeasonCompetitorsBody, SeasonalCompetitorStatistics};
use super::DataProvider;

#[derive(Clone)]
pub struct SportRadarDataProvider {
    api_key: String,
    client: reqwest::Client,
}

impl SportRadarDataProvider {
    pub fn new(api_key: String) -> Self {
        Self {
            api_key,
            client: Default::default(),
        }
    }

    async fn get<T: DeserializeOwned>(&self, path: String) -> anyhow::Result<T> {
        let url = format!("https://api.sportradar.com/soccer/trial/v4/en/{path}");
        let params = [("api_key", &self.api_key)];
        let resp = self
            .client
            .get(url)
            .query(&params)
            .send()
            .await?
            .error_for_status()?;
        Ok(resp.json().await?)
    }
}

#[async_trait::async_trait]
impl DataProvider for SportRadarDataProvider {
    async fn get_season_competitors(
        &self,
        season_id: &str,
    ) -> anyhow::Result<ResponseEnvelop<SeasonCompetitorsBody>> {
        self.get(format!("seasons/{season_id}/competitors.json"))
            .await
    }

    async fn get_season_competitor_statictic(
        &self,
        season_id: &str,
        competitor_id: &str,
    ) -> anyhow::Result<ResponseEnvelop<SeasonalCompetitorStatistics>> {
        self.get(format!(
            "seasons/{season_id}/competitors/{competitor_id}/statistics.json"
        ))
        .await
    }
}
