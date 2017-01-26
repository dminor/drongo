extern crate libc;

pub const SDL_INIT_AUDIO : u32 = 0x10;
pub const AUDIO_F32 : u16 = 0x8120;
pub const AUDIO_S16LSB : u16 = 0x8010;

#[repr(C)]
pub struct SDL_AudioSpec {
  pub freq: i32,
  pub format: u16,
  pub channels: u8,
  pub silence: u8,
  pub samples: u16,
  pub padding: u16,
  pub size: u32,
  pub callback: Option<extern fn(userdata: *mut libc::c_void, stream: *mut u8, len: i32)>,
  pub userdata: *mut libc::c_void,
}

pub enum SDL_RWops {}

#[link(name = "SDL2")]
extern {
  pub fn SDL_Init(flags: u32) -> i32;

  pub fn SDL_OpenAudio(requested: *const SDL_AudioSpec, obtained: *mut SDL_AudioSpec) -> i32;

  pub fn SDL_PauseAudio(pause_on: i32);

  pub fn SDL_LoadWAV_RW(file: *const SDL_RWops, freesrc: i32, spec: *mut SDL_AudioSpec, buffer: *mut *mut u8, length: *mut u32) -> *mut SDL_AudioSpec;

  pub fn SDL_FreeWAV(buffer: *mut u8);

  pub fn SDL_RWFromFile(file: *const libc::c_char, mode: *const libc::c_char) -> *const SDL_RWops;

  pub fn SDL_RWclose(context: *const SDL_RWops);

  pub fn SDL_Quit();
}
