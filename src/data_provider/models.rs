use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct ResponseEnvelop<T> {
    #[serde(flatten)]
    pub body: T,
}

#[derive(Debug, Deserialize)]
pub struct SeasonCompetitorsBody {
    pub season_competitors: Vec<Competitor>,
}

#[derive(Debug, Deserialize)]
pub struct Competitor {
    pub id: String,
    pub name: String,
}

#[derive(Debug, Deserialize)]
pub struct SeasonalCompetitorStatistics {
    pub competitor: CompetitorWithStatistics,
}

#[derive(Debug, Deserialize)]
pub struct CompetitorWithStatistics {
    pub id: String,
    pub name: String,
    pub players: Vec<Player>,
}

#[derive(Debug, Deserialize)]
pub struct Player {
    pub id: String,
    pub name: String,
    pub statistics: PlayerStats,
}

#[derive(Debug, Deserialize)]
pub struct PlayerStats {
    pub assists: u64,
    pub goals_scored: u64,
}
