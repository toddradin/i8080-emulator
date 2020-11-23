use crate::i8080::memory_bus::MemoryMap;

use std::fs::File;
use std::io::Read;

pub const ROM_BEGIN: usize = 0x0000;
pub const ROM_END: usize = 0x47FF;
pub const ROM_SIZE: usize = ROM_END - ROM_BEGIN + 1;

pub const WORKING_RAM_BEGIN: usize = 0x2000;
pub const WORKING_RAM_END: usize = 0x23FF;
pub const WORKING_RAM_SIZE: usize = WORKING_RAM_END - WORKING_RAM_BEGIN + 1;

pub const VIDEO_RAM_BEGIN: usize = 0x2400;
pub const VIDEO_RAM_END: usize = 0x3FFF;
pub const VIDEO_RAM_SIZE: usize = VIDEO_RAM_END - VIDEO_RAM_BEGIN + 1;

pub const RAM_MIRROR_BEGIN: usize = 0x4800;
pub const RAM_MIRROR_END: usize = 0x87FF;

pub struct BalloonBombersMemory {
    rom: [u8; ROM_SIZE],
    working_ram: [u8; WORKING_RAM_SIZE],
    video_ram: [u8; VIDEO_RAM_SIZE],
}

impl BalloonBombersMemory {
    pub fn new() -> Self {
        let buffer = [0; ROM_SIZE];
        let mut memory = Self {
            rom: buffer,
            working_ram: [0; WORKING_RAM_SIZE],
            video_ram: [0; VIDEO_RAM_SIZE],
        };
        memory.load_rom();
        memory
    }
}

impl MemoryMap for BalloonBombersMemory {
    fn load_rom(&mut self) {
        let mut addr = 0x00;
        for f in ['1', '2', '3', '4'].iter() {
            let mut file = File::open(format!("roms/tn0{}", f)).unwrap();
            file.read(&mut self.rom[addr..addr + 0x800]).unwrap();
            addr += 0x800;
        }
           
        addr = 0x4000;
        let mut file = File::open("roms/tn05-1").unwrap();
        file.read(&mut self.rom[addr..addr + 0x800]).unwrap();
    }

    fn read(&mut self, addr: u16) -> u8 {
        let addr = addr as usize;
        match addr {
            WORKING_RAM_BEGIN..=WORKING_RAM_END => self.working_ram[addr - WORKING_RAM_BEGIN],
            VIDEO_RAM_BEGIN..=VIDEO_RAM_END => self.video_ram[addr - VIDEO_RAM_BEGIN],
            ROM_BEGIN..=ROM_END => self.rom[addr],
            RAM_MIRROR_BEGIN..=RAM_MIRROR_END => self.working_ram[addr - RAM_MIRROR_BEGIN],
            _ => panic!(
                "Attempting to read from an unknown area of memory: {:#x?}",
                addr
            ),
        }
    }

    fn read_slice(&mut self, addr: u16) -> &[u8] {
        let addr = addr as usize;
        match addr {
            WORKING_RAM_BEGIN..=WORKING_RAM_END => &self.working_ram[addr - WORKING_RAM_BEGIN..],
            VIDEO_RAM_BEGIN..=VIDEO_RAM_END => &self.video_ram[addr - VIDEO_RAM_BEGIN..],
            ROM_BEGIN..=ROM_END => &self.rom[addr..ROM_END],
            RAM_MIRROR_BEGIN..=RAM_MIRROR_END => {
                &self.working_ram[addr - RAM_MIRROR_BEGIN..]
            }
            _ => panic!(
                "Attempting to read from an unknown area of memory: {:#x?}",
                addr
            ),
        }
    }

    fn write(&mut self, addr: u16, val: u8) {
        let addr = addr as usize;
        match addr {
            WORKING_RAM_BEGIN..=WORKING_RAM_END => self.working_ram[addr - WORKING_RAM_BEGIN] = val,
            VIDEO_RAM_BEGIN..=VIDEO_RAM_END => self.video_ram[addr - VIDEO_RAM_BEGIN] = val,
            _ => (),
        }
    }
}
