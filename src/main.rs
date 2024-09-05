use std::time::Duration;

use clap::Parser;
use futures::stream::{self, StreamExt};
use futures::TryStreamExt;
use output::console::print_table;

use crate::data_provider::sport_radar::SportRadarDataProvider;
use crate::data_provider::DataProvider;
use crate::statistics::top_assisted_players::top_assisted_players;
use crate::statistics::top_scored_players::top_scored_players;

pub mod data_provider;
pub mod output;
pub mod statistics;

/// Simple program for fetching top scored and assisted players using Sportradar API
#[derive(Debug, Parser)]
#[command(about)]
struct Args {
    /// Sportradar api token
    #[arg(short, long)]
    api_token: String,

    /// Season ID to retrieve data for
    #[arg(short, long, default_value = "sr:season:105353")]
    season_id: String,

    /// Number of records displayed
    #[arg(short, long, default_value_t = 10)]
    output_size: usize,

    /// Don't send more than 1 RPS. Only useful for trial api tokens
    #[arg(short, long, default_value_t = false)]
    enable_rate_limit: bool,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    let data_provider = SportRadarDataProvider::new(args.api_token);
    let competitors = data_provider
        .get_season_competitors(&args.season_id)
        .await?;

    let competitors: Vec<_> = stream::iter(competitors.body.season_competitors)
        .then(move |competitor| {
            let data_provider = data_provider.clone();
            let season_id = args.season_id.clone();
            async move {
                // Added to support trial api keys that have 1 request per second limit.
                // I found it more convenient to use competitor statistics api to get information
                // about players in a given season, but it doesn't go well with trial keys rate limit.
                // Alternatively, Season Summaries endpoint can be used to derive the same data using single api request.
                if args.enable_rate_limit {
                    tokio::time::sleep(Duration::from_secs(1)).await;
                }

                data_provider
                    .get_season_competitor_statictic(&season_id, &competitor.id)
                    .await
                    .map(|resp| resp.body.competitor)
            }
        })
        .try_collect::<Vec<_>>()
        .await?;

    let top_assisted_players = top_assisted_players(&competitors);
    print_table(
        "Top assisted players",
        top_assisted_players,
        args.output_size,
    );

    let top_scored_players = top_scored_players(&competitors);
    print_table("Top scored players", top_scored_players, args.output_size);

    Ok(())
}
