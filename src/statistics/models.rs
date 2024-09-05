use tabled::Tabled;

#[derive(Debug, Tabled)]
pub struct AssistedPlayer {
    pub id: String,
    pub name: String,
    pub team_id: String,
    pub team_name: String,
    pub assists: u64,
}

#[derive(Debug, Tabled)]
pub struct ScoredPlayer {
    pub id: String,
    pub name: String,
    pub team_id: String,
    pub team_name: String,
    pub scores: u64,
}
