use super::models::ScoredPlayer;
use crate::data_provider::models::CompetitorWithStatistics;

pub fn top_scored_players(competitors: &[CompetitorWithStatistics]) -> Vec<ScoredPlayer> {
    let mut scored_players: Vec<_> = competitors
        .iter()
        .flat_map(|competitor| {
            competitor.players.iter().map(move |player| ScoredPlayer {
                id: player.id.clone(),
                name: player.name.clone(),
                team_id: competitor.id.clone(),
                team_name: competitor.name.clone(),
                scores: player.statistics.goals_scored,
            })
        })
        .collect();

    scored_players.sort_by(|l, r| r.scores.cmp(&l.scores));
    scored_players
}
