use crate::i8080::memory_bus::MemoryMap;

use std::fs::File;
use std::io::Read;

pub const ROM_BEGIN: usize = 0x0000;
pub const ROM_END: usize = 0x1FFF;
pub const ROM_SIZE: usize = ROM_END - ROM_BEGIN + 1;

pub const WORKING_RAM_BEGIN: usize = 0x2000;
pub const WORKING_RAM_END: usize = 0x23FF;
pub const WORKING_RAM_SIZE: usize = WORKING_RAM_END - WORKING_RAM_BEGIN + 1;

pub const VIDEO_RAM_BEGIN: usize = 0x2400;
pub const VIDEO_RAM_END: usize = 0x3FFF;
pub const VIDEO_RAM_SIZE: usize = VIDEO_RAM_END - VIDEO_RAM_BEGIN + 1;

pub const RAM_MIRROR_BEGIN: usize = 0x4000;
pub const RAM_MIRROR_END: usize = 0xFFFF;
pub const RAM_MIRROR_SIZE: usize = RAM_MIRROR_END - RAM_MIRROR_BEGIN + 1;

pub struct SpaceInvadersMemory {
    rom: [u8; ROM_SIZE],
    working_ram: [u8; WORKING_RAM_SIZE],
    video_ram: [u8; VIDEO_RAM_SIZE],
    ram_mirror: [u8; RAM_MIRROR_SIZE],
}

impl SpaceInvadersMemory {
    pub fn new() -> Self {
        let mut buffer = [0; ROM_SIZE];
        SpaceInvadersMemory::load_rom(&mut buffer);
        Self {
            rom: buffer,
            working_ram: [0; WORKING_RAM_SIZE],
            video_ram: [0; VIDEO_RAM_SIZE],
            ram_mirror: [0; RAM_MIRROR_SIZE],
        }
    }
}

impl MemoryMap for SpaceInvadersMemory {
    fn load_rom(buffer: &mut [u8]) {
        let mut addr = 0x00;
        for f in ['h', 'g', 'f', 'e'].iter() {
            let mut file = File::open(format!("roms/invaders.{}", f)).unwrap();
            file.read(&mut buffer[addr..addr + 0x800]).unwrap();
            addr += 0x800;
        }
    }

    fn read(&mut self, addr: u16) -> u8 {
        let addr = addr as usize;
        match addr {
            ROM_BEGIN..=ROM_END => self.rom[addr],
            WORKING_RAM_BEGIN..=WORKING_RAM_END => self.working_ram[addr - WORKING_RAM_BEGIN],
            VIDEO_RAM_BEGIN..=VIDEO_RAM_END => self.video_ram[addr - VIDEO_RAM_BEGIN],
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
            ROM_BEGIN..=ROM_END => &self.rom[addr..ROM_END],
            WORKING_RAM_BEGIN..=WORKING_RAM_END => &self.working_ram[addr - WORKING_RAM_BEGIN..],
            VIDEO_RAM_BEGIN..=VIDEO_RAM_END => &self.video_ram[addr - VIDEO_RAM_BEGIN..],
            RAM_MIRROR_BEGIN..=RAM_MIRROR_END => {
                // &self.ram_mirror[addr - RAM_MIRROR_BEGIN..RAM_MIRROR_BEGIN]
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
