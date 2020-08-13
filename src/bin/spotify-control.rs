extern crate mpris;

use clap::{App, ArgMatches};
use mpris::{PlayerFinder, Metadata};

fn main() {
    let matches: ArgMatches = App::new("spotify-control")
        .version("0.1.0")
        .author("Mathieu Santostefano <msantostefano@protonmail.com>")
        .about("Controls Spotify from CLI, retrieve current playing data")
        .subcommand(App::new("play").about("send play command to Spotify"))
        .subcommand(App::new("pause").about("send pause command to Spotify"))
        .subcommand(App::new("play_pause").about("send play_pause command to Spotify"))
        .subcommand(App::new("next").about("send next command to Spotify"))
        .subcommand(App::new("previous").about("send previous command to Spotify"))
        .subcommand(App::new("status").about("fetch current playing metadata"))
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
        Some("play_pause") => {
            player.play_pause().expect("Could not play_pause");
        }
        Some("next") => {
            player.next().expect("Could not next");
        }
        Some("previous") => {
            player.previous().expect("Could not previous");
        }
        Some("status") => {
            let metadata: Metadata = player
                .get_metadata()
                .expect("Could not get metadata for player");
            let artist: String = metadata.artists().unwrap()[0].to_string();
            let track: String = metadata.title().unwrap().to_string();
            let mut status: String = artist + " - " + &track;

            if status.len() > 80 {
                status.truncate(80);
                status.push_str("...");
            }

            println!("{}", status.to_string());
        }
        None => unreachable!(),
        _ => unreachable!(),
    }
}
