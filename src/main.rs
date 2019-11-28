// use futures_util::{pin_mut, stream::StreamExt};
use hyper::Client;
use hyper_rustls::HttpsConnector;
// use std::{fs::File, io::Write};

#[tokio::main]
async fn main() {
    if let Err(e) = try_run().await {
        println!("{:?}", e);
    }
}

async fn try_run() -> anyhow::Result<()> {
    let client = Client::builder().build(HttpsConnector::new());
    // let run = speedrun_com_api::run::get(&client, String::from("z0332rjz")).await?;
    // dbg!(run);

    let game = speedrun_com_api::games::by_id(&client, String::from("4d709l17")).await?;
    dbg!(game);

    // let search = speedrun_com_api::games::search(&client, String::from("Mario"));
    // pin_mut!(search);
    // while let Some(game) = search.next().await {
    //     dbg!(game?);
    // }

    // let mut file = File::create("all_games.txt").unwrap();
    // let all_games = speedrun_com_api::games::all(&client, None);
    // pin_mut!(all_games);
    // while let Some(game) = all_games.next().await {
    //     writeln!(file, "{}", game?.names.international).unwrap();
    // }

    Ok(())
}
