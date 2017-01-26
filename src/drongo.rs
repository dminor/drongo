extern crate libc;
extern crate rand;
extern crate std;

use std::collections::HashMap;
use std::ffi::CString;

use sdl;

// Sample frequency in Hz
const F_SAMPLE : usize = 44100;

struct Channel {
  offset: usize,
  playing: bool,
  vol: f32,
  samples: Vec<f32>,
  beat_mask: u16,
}

struct Sound {
  samples: Vec<f32>,
}

pub struct AudioEngine {
  channels: Vec<Channel>,
  sounds: HashMap<String, Sound>,
  next_channel_id: i32,
  beat : usize,
  next_beat_timestamp : usize,
  timestamp : usize,
  tempo : usize,
}

impl AudioEngine {
  pub fn new(tempo : usize) -> AudioEngine {
    let engine = AudioEngine {
      channels: Vec::new(),
      sounds: HashMap::new(),
      next_channel_id: 0,
      beat: 0,
      next_beat_timestamp: F_SAMPLE * 60 / tempo,
      timestamp: 0,
      tempo : tempo
    };
    engine
  }

  pub fn init(&mut self) {
    let requested = sdl::SDL_AudioSpec {
      freq: F_SAMPLE as i32,
      format: sdl::AUDIO_S16LSB,
      channels: 2,
      silence: 0,
      samples: 512,
      padding: 0,
      size: 0,
      callback: Some(audio_callback),
      userdata: self as *mut _ as *mut libc::c_void,
    };

    let mut obtained = sdl::SDL_AudioSpec {.. requested};

    unsafe {
      let mut res = sdl::SDL_Init(sdl::SDL_INIT_AUDIO);
      res = sdl::SDL_OpenAudio(&requested, &mut obtained);
      sdl::SDL_PauseAudio(0);
    }
  }

  pub fn update(&mut self, buffer: &mut [i16]) {
    let mut output = vec![0.0; buffer.len()];

    self.timestamp += buffer.len() / 2;
    if self.timestamp >= self.next_beat_timestamp {
      self.next_beat_timestamp += F_SAMPLE * 60 / self.tempo;
      self.beat += 1;
      if self.beat >= 16 {
        self.beat = 0;
      }

      let beat_mask = 1 << self.beat;

      // Update channel state for new beat
      for i in 0 .. self.channels.len() {
        if self.channels[i].beat_mask & beat_mask == beat_mask {
          self.channels[i].playing = true;
          self.channels[i].offset = 0;
        }
      }
    }

    for i in 0 .. self.channels.len() {
      if !self.channels[i].playing {
        continue;
      }

      for j in 0 .. buffer.len() {
        let mut idx = self.channels[i].offset;

        if idx >= self.channels[i].samples.len() {
          self.channels[i].playing = false;
          self.channels[i].offset = 0;
          break;
        }
        output[j] += self.channels[i].samples[idx] * self.channels[i].vol;
        self.channels[i].offset += 1;
      }
    }

    for i in 0 .. buffer.len() {
      buffer[i] = (output[i] * 32767.0) as i16;
    }
  }

  pub fn shutdown(&self) {
    unsafe {
      sdl::SDL_PauseAudio(1);
      sdl::SDL_Quit();
    }
  }

  fn load_sound(&mut self, name: &str) {
    if self.sounds.contains_key(name) {
      return
    }

    let path = CString::new(name).unwrap();
    let mode = CString::new("rb").unwrap();
    unsafe {
      let rwops = sdl::SDL_RWFromFile(path.as_ptr(), mode.as_ptr());

      let mut spec = sdl::SDL_AudioSpec {
        freq: F_SAMPLE as i32,
        format: sdl::AUDIO_S16LSB,
        channels: 2,
        silence: 0,
        samples: 0,
        padding: 0,
        size: 0,
        callback: None,
        userdata: 0 as *mut libc::c_void,
      };

      let mut u : u8 = 0;
      let mut buffer : *mut u8 = &mut u;
      let mut length : u32 = 0;

      sdl::SDL_LoadWAV_RW(rwops, 1, &mut spec, &mut buffer, &mut length);

      let mut sound = Sound {
        samples: Vec::new()
      };

      if length > 0 {
        let samples = std::slice::from_raw_parts_mut(buffer as *mut i16, (length / 2) as usize);
        for i in 1 .. samples.len() {
          sound.samples.push(samples[i] as f32 / 32768.0)
        }
        sdl::SDL_FreeWAV(buffer);
      }

      self.sounds.insert(name.to_string(), sound);
    }
  }

  pub fn unload_sound(&mut self, name: &str) {
    self.sounds.remove(&name.to_string());
  }

  pub fn play_sound(&mut self, name: &str, vol : f32, beat_mask : u16) -> i32 {

    let id = self.next_channel_id;
    self.next_channel_id = self.next_channel_id + 1;


    if !self.sounds.contains_key(name) {
      self.load_sound(name)
    }

    let ref sound = self.sounds[name];

    let mut channel = Channel {
      offset: 0,
      playing: true,
      vol: vol,
      samples: Vec::new(),
      beat_mask: beat_mask,
    };

    channel.samples.extend(&sound.samples);

    self.channels.push(channel);

    id
  }

  fn stop_channel(&mut self, chan: usize) {
    if chan < self.channels.len() {
      self.channels[chan].playing = false;
    }
  }

  fn stop_all_channels(&mut self) {
    for i in 1 .. self.channels.len() {
      self.channels[i].playing = false;
    }
  }

  fn set_channel_volume(&mut self, chan: usize, vol: f32) {
    if chan < self.channels.len() {
      self.channels[chan].vol = vol;
    }
  }

  fn channel_playing(&self, chan: usize) -> bool {
    if chan < self.channels.len() {
      return self.channels[chan].playing;
    }
    false
  }
}

extern "C" fn audio_callback(userdata: *mut libc::c_void, stream: *mut u8, len: i32) {
  unsafe {
    let engine = userdata as *mut AudioEngine;

    let fstream = std::slice::from_raw_parts_mut(stream as *mut i16, (len / 2) as usize);

    (*engine).update(fstream);
  }
}
