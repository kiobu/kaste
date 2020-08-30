use spotifyqdbus::*;
use clap::{Arg, App};
fn main() {
    let matches = App::new("Example")
        .author("kiobu")
        .about("Mange your Spotify through the terminal.")
        .arg(Arg::new("play")
            .long("play")
            .required(false))
        .arg(Arg::new("pause")
            .long("pause")
            .required(false))
        .get_matches();
        
    if matches.is_present("play") {
        spotify::exec("Play");
    } else if matches.is_present("pause") {
        spotify::exec("Pause");
    }
}