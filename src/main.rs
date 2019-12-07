use futures_util::{pin_mut, stream::StreamExt};
use hyper::Client;
use hyper_rustls::HttpsConnector;
use livesplit_core::timing::{
    formatter::{Short, TimeFormatter},
    TimeSpan,
};
use snafu::{OptionExt, ResultExt};
use speedrun_com_api::{categories, games, leaderboards};

#[derive(Debug, snafu::Snafu)]
enum Error {
    /// Couldn't find Wind Waker.
    FindGame,
    /// Failed downloading the game information for Wind Waker.
    Game { source: speedrun_com_api::Error },
    /// Failed downloading Wind Waker's categories.
    Categories { source: speedrun_com_api::Error },
    #[snafu(display("Failed accessing the leaderboard for {}.", category))]
    Leaderboard {
        source: speedrun_com_api::Error,
        category: String,
    },
}

#[tokio::main]
async fn main() {
    if let Err(e) = try_run().await {
        println!("{:?}", e);
    }
}

async fn try_run() -> anyhow::Result<()> {
    let client = Client::builder().build(HttpsConnector::new());

    let search = games::search(&client, String::from("Wind Waker"));
    pin_mut!(search);
    let game = search.next().await.context(FindGame)?.context(Game)?;

    let categories = categories::for_game(&client, game.id.clone())
        .await
        .context(Categories)?;

    termimad::print_text(&format!("# {}\n", game.names.international));

    for category in categories {
        termimad::print_text(&format!("\n## {}\n\n", category.name));
        if let Some(rules) = &category.rules {
            termimad::print_text(rules);
        }
        let leaderboard = leaderboards::get(
            &client,
            game.id.clone(),
            category.id,
            leaderboards::Embeds::PLAYERS,
        )
        .await
        .context(Leaderboard {
            category: category.name,
        })?;

        if let Some(run) = leaderboard.runs.get(0) {
            println!(
                "WR is {}",
                Short::new().format(TimeSpan::from_seconds(run.run.times.primary_t))
            );
        }
    }

    Ok(())
}
