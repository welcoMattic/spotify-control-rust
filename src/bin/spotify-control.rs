extern crate mpris;

use clap::App;
use mpris::PlayerFinder;

fn main() {
    let matches = App::new("spotify-control")
        .version("0.1")
        .author("Mathieu Santostefano <msantostefano@protonmail.com>")
        .about("Controls Spotify from CLI, retrieve current playing data")
        .subcommand(App::new("play").about("send play command to Spotify"))
        .subcommand(App::new("pause").about("send pause command to Spotify"))
        .subcommand(App::new("metadata").about("fetch current playing metadata"))
        .get_matches();

    let player = PlayerFinder::new()
        .expect("Could not connect to D-Bus")
        .find_active()
        .expect("Could not find any player");

    match matches.subcommand_name() {
        Some("play") => {
            player.play().expect("Could not play");
        }
        Some("pause") => {
            player.pause().expect("Could not pause");
        }
        Some("metadata") => {
            let metadata = player
                .get_metadata()
                .expect("Could not get metadata for player");
            println!("{:#?}", metadata);
        }
        None => unreachable!(),
        _ => unreachable!(),
    }
}
