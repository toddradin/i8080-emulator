#[macro_use]
extern crate bitflags;
extern crate i8080;
extern crate sdl2;
use crate::display::Display;
use crate::io::{ControllerPort, Key, SpaceInvadersIO};
use crate::memory::SpaceInvadersMemory;

mod display;
mod io;
mod memory;

use i8080::cpu::Cpu;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::audio::{AudioCallback, AudioSpecDesired, AudioSpecWAV, AudioCVT};

struct Sound {
    data: Vec<u8>,
    volume: f32,
    position: usize,
}

impl AudioCallback for Sound {
    type Channel = u8;

    fn callback(&mut self, out: &mut [u8]) {
        //Referenced from Rust SDL2 documentation:
        //https://github.com/Rust-SDL2/rust-sdl2/blob/master/examples/audio-wav.rs
        for dst in out.iter_mut() {
            //required for scaling the volume properly
            let pre_scale = *self.data.get(self.position).unwrap_or(&128);
            let scaled_signed_float = (pre_scale as f32 - 128.0) * self.volume;
            *dst = (scaled_signed_float + 128.0) as u8;
            self.position += 1;
        }
    }
}

fn keycode_to_key(keycode: Keycode) -> Option<(Key, ControllerPort)> {
    let key = match keycode {
        Keycode::Num0 => (Key::CREDIT, ControllerPort::P1),
        Keycode::Num2 => (Key::START2P, ControllerPort::P1),
        Keycode::Num1 => (Key::START1P, ControllerPort::P1),
        Keycode::W => (Key::SHOOT1P, ControllerPort::P1),
        Keycode::A => (Key::LEFT1P, ControllerPort::P1),
        Keycode::D => (Key::RIGHT1P, ControllerPort::P1),
        Keycode::I => (Key::SHOOT2P, ControllerPort::P2),
        Keycode::J => (Key::LEFT2P, ControllerPort::P2),
        Keycode::L => (Key::RIGHT2P, ControllerPort::P2),
        _ => return None,
    };

    Some(key)
}

fn main() -> Result<(), std::io::Error> {
    let memory = SpaceInvadersMemory::new();
    let machine = &mut SpaceInvadersIO::new();
    let cpu = &mut Cpu::new(memory);

    let sdl_context = sdl2::init().unwrap();
    let audio_subsystem = sdl_context.audio().unwrap();
    let mut event_pump = sdl_context.event_pump().unwrap();
    let mut display = Display::new(sdl_context);

    let desired_spec = AudioSpecDesired {
        freq: Some(11025),
        channels: Some(1),
        samples: None
    };

    const HERTZ: i32 = 2_000_000;
    const FPS: u8 = 60;
    const CYCLES_PER_FRAME: i32 = HERTZ / FPS as i32;
    const CYCLES_PER_HALF_FRAME: i32 = CYCLES_PER_FRAME / 2;

    let mut next_interrupt = 0x8;

    //setting up some audio variables before main game loop
    let mut ufo = audio_subsystem.open_playback(None, &desired_spec, |spec| {
        //audio_subsytem handling here and below referenced from official documentation: 
        //https://github.com/Rust-SDL2/rust-sdl2/blob/master/examples/audio-wav.rs
        let wav = AudioSpecWAV::load_wav("./sfx/ufo_long.wav").unwrap();
        let cvt = AudioCVT::new(wav.format, wav.channels, wav.freq, spec.format, spec.channels, spec.freq).expect("Could not convert WAV file");
        let data = cvt.convert(wav.buffer().to_vec());
        Sound {
        data: data,
        volume: 0.20,
        position: 0,
    }
    }).unwrap();
    let mut ufo_reset = 0; //keeps track if ufo fx needs to be set again

    let mut mainfx = audio_subsystem.open_playback(None, &desired_spec, |spec| {
        let wav = AudioSpecWAV::load_wav("./sfx/shoot.wav").unwrap();
        let cvt = AudioCVT::new(wav.format, wav.channels, wav.freq, spec.format, spec.channels, spec.freq).expect("Could not convert WAV file");
        let data = cvt.convert(wav.buffer().to_vec());
        Sound {
        data: data,
        volume: 0.20,
        position: 0,
    }
    }).unwrap();

    let mut invadersfx = audio_subsystem.open_playback(None, &desired_spec, |spec| {
        let wav = AudioSpecWAV::load_wav("./sfx/shoot.wav").unwrap();
        let cvt = AudioCVT::new(wav.format, wav.channels, wav.freq, spec.format, spec.channels, spec.freq).expect("Could not convert WAV file");
        let data = cvt.convert(wav.buffer().to_vec());
        Sound {
        data: data,
        volume: 0.20,
        position: 0,
    }
    }).unwrap();

    let mut explode = audio_subsystem.open_playback(None, &desired_spec, |spec| {
        let wav = AudioSpecWAV::load_wav("./sfx/shoot.wav").unwrap();
        let cvt = AudioCVT::new(wav.format, wav.channels, wav.freq, spec.format, spec.channels, spec.freq).expect("Could not convert WAV file");
        let data = cvt.convert(wav.buffer().to_vec());
        Sound {
        data: data,
        volume: 0.20,
        position: 0,
    }
    }).unwrap();

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                Event::KeyDown {
                    keycode: Some(keycode),
                    ..
                } => {
                    if let Some((key, port)) = keycode_to_key(keycode) {
                        machine.press(key, port);
                    }
                }
                Event::KeyUp {
                    keycode: Some(keycode),
                    ..
                } => {
                    if let Some((key, port)) = keycode_to_key(keycode) {
                        machine.release(key, port);
                    }
                }
                _ => {}
            }
        }

        //AUDIO
        //making sure that the UFO sound effect is always set up to be played 
        if ufo_reset == 0 {
            ufo = audio_subsystem.open_playback(None, &desired_spec, |spec| {
                let wav = AudioSpecWAV::load_wav("./sfx/ufo_long.wav").unwrap();
                let cvt = AudioCVT::new(wav.format, wav.channels, wav.freq, spec.format, spec.channels, spec.freq).expect("Could not convert WAV file");
                let data = cvt.convert(wav.buffer().to_vec());
                Sound {
                data: data,
                volume: 0.20,
                position: 0,
            }
            }).unwrap();
            ufo_reset = 1;

        }

        if machine.third_port != machine.prev_third_port {
            //UFO START
            if ((machine.third_port & 0x1) != 0) && (!(machine.prev_third_port & 0x1) != 0) {
                ufo.resume(); //UFO sfx plays until 'UFO STOP' is called below
            }

            //UFO STOP
            else if (!(machine.third_port & 0x1) != 0) && (!(machine.prev_third_port & 0x1) != 0) {
                ufo.pause(); //stops the effect
                ufo_reset = 0; //flag for resetting the UFO sfx to be played again later
            }
                      
            //SHOOT
            if ((machine.third_port & 0x2) != 0) && (!(machine.prev_third_port & 0x2) != 0) {
                mainfx = audio_subsystem.open_playback(None, &desired_spec, |spec| {
                    let wav = AudioSpecWAV::load_wav("./sfx/shoot.wav").unwrap();
                    let cvt = AudioCVT::new(wav.format, wav.channels, wav.freq, spec.format, spec.channels, spec.freq).expect("Could not convert WAV file");
                    let data = cvt.convert(wav.buffer().to_vec());
                    Sound {
                    data: data,
                    volume: 0.20,
                    position: 0,
                }
                }).unwrap();
                mainfx.resume();
            }

            //PLAYER DEATH
            if ((machine.third_port & 0x4) != 0) && (!(machine.prev_third_port & 0x4) != 0) {
                mainfx = audio_subsystem.open_playback(None, &desired_spec, |spec| {
                    let wav = AudioSpecWAV::load_wav("./sfx/explosion.wav").unwrap();
                    let cvt = AudioCVT::new(wav.format, wav.channels, wav.freq, spec.format, spec.channels, spec.freq).expect("Could not convert WAV file");
                    let data = cvt.convert(wav.buffer().to_vec());
                    Sound {
                    data: data,
                    volume: 0.20,
                    position: 0,
                }
                }).unwrap();
                mainfx.resume();
            }

            //INVADER DEATH
            if ((machine.third_port & 0x8) != 0) && (!(machine.prev_third_port & 0x8) != 0) {
                explode = audio_subsystem.open_playback(None, &desired_spec, |spec| {
                    let wav = AudioSpecWAV::load_wav("./sfx/invaderkilled.wav").unwrap();
                    let cvt = AudioCVT::new(wav.format, wav.channels, wav.freq, spec.format, spec.channels, spec.freq).expect("Could not convert WAV file");
                    let data = cvt.convert(wav.buffer().to_vec());
                    Sound {
                    data: data,
                    volume: 0.20,
                    position: 0,
                }
                }).unwrap();
                explode.resume();
            }
            machine.prev_third_port = machine.third_port;
        }    

        if machine.fifth_port != machine.prev_fifth_port {
            //INVADER 1
            if ((machine.fifth_port & 0x1) != 0) && (!(machine.prev_fifth_port & 0x1) != 0) {
                invadersfx = audio_subsystem.open_playback(None, &desired_spec, |spec| {
                    let wav = AudioSpecWAV::load_wav("./sfx/fastinvader4.wav").unwrap();
                    let cvt = AudioCVT::new(wav.format, wav.channels, wav.freq, spec.format, spec.channels, spec.freq).expect("Could not convert WAV file");
                    let data = cvt.convert(wav.buffer().to_vec());
                    Sound {
                    data: data,
                    volume: 0.20,
                    position: 0,
                }
                }).unwrap();
                invadersfx.resume();
            }

            //INVADER 2
            if ((machine.fifth_port & 0x2) != 0) && (!(machine.prev_fifth_port & 0x2) != 0) {
                invadersfx = audio_subsystem.open_playback(None, &desired_spec, |spec| {
                    let wav = AudioSpecWAV::load_wav("./sfx/fastinvader1.wav").unwrap();
                    let cvt = AudioCVT::new(wav.format, wav.channels, wav.freq, spec.format, spec.channels, spec.freq).expect("Could not convert WAV file");
                    let data = cvt.convert(wav.buffer().to_vec());
                    Sound {
                    data: data,
                    volume: 0.20,
                    position: 0,
                }
                }).unwrap();
                invadersfx.resume();
            }

            //INVADER 3
            if ((machine.fifth_port & 0x4) != 0) && (!(machine.prev_fifth_port & 0x4) != 0) {
                invadersfx = audio_subsystem.open_playback(None, &desired_spec, |spec| {
                    let wav = AudioSpecWAV::load_wav("./sfx/fastinvader2.wav").unwrap();
                    let cvt = AudioCVT::new(wav.format, wav.channels, wav.freq, spec.format, spec.channels, spec.freq).expect("Could not convert WAV file");
                    let data = cvt.convert(wav.buffer().to_vec());
                    Sound {
                    data: data,
                    volume: 0.20,
                    position: 0,
                }
                }).unwrap();
                invadersfx.resume();
            }

            //INVADER 4
            if ((machine.fifth_port & 0x8) != 0) && (!(machine.prev_fifth_port & 0x8) != 0) {
                invadersfx = audio_subsystem.open_playback(None, &desired_spec, |spec| {
                    let wav = AudioSpecWAV::load_wav("./sfx/fastinvader3.wav").unwrap();
                    let cvt = AudioCVT::new(wav.format, wav.channels, wav.freq, spec.format, spec.channels, spec.freq).expect("Could not convert WAV file");
                    let data = cvt.convert(wav.buffer().to_vec());
                    Sound {
                    data: data,
                    volume: 0.20,
                    position: 0,
                }
                }).unwrap();
                invadersfx.resume();
            }

            //UFO EXPLOSION
            if ((machine.fifth_port & 0x10) != 0) && (!(machine.prev_fifth_port & 0x10) != 0) {
                explode = audio_subsystem.open_playback(None, &desired_spec, |spec| {
                    let wav = AudioSpecWAV::load_wav("./sfx/invaderkilled.wav").unwrap();
                    let cvt = AudioCVT::new(wav.format, wav.channels, wav.freq, spec.format, spec.channels, spec.freq).expect("Could not convert WAV file");
                    let data = cvt.convert(wav.buffer().to_vec());
                    Sound {
                    data: data,
                    volume: 0.20,
                    position: 0,
                }
                }).unwrap();
                explode.resume();
            }
            machine.prev_fifth_port = machine.fifth_port;
        }    

        // After every CYCLES_PER_HALF_FRAME, an interrupt should be triggered.
        // This will be run twice so that the correct number of cycles per
        // frame is reached.
        for _ in 0..2 {
            cpu.step(machine, CYCLES_PER_HALF_FRAME);
            cpu.interrupt(next_interrupt);
            next_interrupt = if next_interrupt == 0x08 { 0x10 } else { 0x08 };
        }
        display.draw_display_whole(cpu);
    }

    Ok(())
}
