pub mod spotify {
    use std::process::Command;
    use colored::*;

    pub fn get_client() -> &'static str {
        return "spotify";
    }

    pub fn exec(action: &str) -> () {
        match Command::new("sh")
            .arg("-c")
            .arg(format!("qdbus org.mpris.MediaPlayer2.{} /org/mpris/MediaPlayer2 org.mpris.MediaPlayer2.Player.{}", get_client(), action))
            .status() 
        { 
            Ok(_status) => (),
            Err(_) => println!("{}", "Unable to execute.".red())
        };
    }
    #[cfg(test)]
    pub mod tests {
        #[test]
        fn pause() {
            super::exec("Pause");
        }
        #[test]
        fn play() {
            super::exec("Play");
        }
    }
}