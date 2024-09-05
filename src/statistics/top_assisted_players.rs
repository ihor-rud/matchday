use super::models::AssistedPlayer;
use crate::data_provider::models::CompetitorWithStatistics;

pub fn top_assisted_players(competitors: &[CompetitorWithStatistics]) -> Vec<AssistedPlayer> {
    let mut assisted_players: Vec<_> = competitors
        .iter()
        .flat_map(|competitor| {
            competitor.players.iter().map(move |player| AssistedPlayer {
                id: player.id.clone(),
                name: player.name.clone(),
                team_id: competitor.id.clone(),
                team_name: competitor.name.clone(),
                assists: player.statistics.assists,
            })
        })
        .collect();

    assisted_players.sort_by(|l, r| r.assists.cmp(&l.assists));
    assisted_players
}
