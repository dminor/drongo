use std::io;
use std::io::Read;

pub mod drongo;
pub mod sdl;

fn main() {

  let mut engine = drongo::AudioEngine::new(140);
  engine.init();
  engine.play_sound("sounds/cowbell_01.wav", 0.3, 43351);
  engine.play_sound("sounds/kick_04.wav", 0.4, 34952);
  engine.play_sound("sounds/hihat_01.wav", 0.3, 65535);
  engine.play_sound("sounds/snare_03.wav", 0.3, 8719);

  let mut buf = [0; 1];
  loop {
    println!("press enter to quit");
    let amt: usize = match io::stdin().read(&mut buf) {
        Ok(amt) => amt,
        Err(_) => continue
    };

    if amt > 0 {
      break;
    }
  }

  engine.shutdown();
}
