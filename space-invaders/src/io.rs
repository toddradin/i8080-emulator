use i8080::cpu::Cpu;
use i8080::machine::MachineIO;
use i8080::memory_bus::MemoryMap;

bitflags! {
    pub struct Key: u8 {
        const CREDIT = 1 << 0;
        const START2P = 1 << 1;
        const START1P = 1 << 2;
        const SHOOT1P = 1 << 4;
        const LEFT1P = 1 << 5;
        const RIGHT1P = 1 << 6;
        const SHOOT2P = 1 << 4;
        const LEFT2P = 1 << 5;
        const RIGHT2P = 1 << 6;
    }
}
pub enum ControllerPort {
    P1,
    P2,
}

pub struct SpaceInvadersIO {
    first_port: u8,
    second_port: u8,
    shift0: u8,
    shift1: u8,
    shift_offset: u8,
    pub third_port: u8,
    pub prev_third_port: u8,
    pub fifth_port: u8,
    pub prev_fifth_port: u8,
}

impl SpaceInvadersIO {
    pub fn new() -> SpaceInvadersIO {
        SpaceInvadersIO {
            first_port: 1,
            second_port: 0,
            shift0: 0,
            shift1: 0,
            shift_offset: 0,
            third_port: 0,
            prev_third_port: 0,
            fifth_port: 0,
            prev_fifth_port: 0,
        }
    }
}

impl MachineIO for SpaceInvadersIO {
    fn machine_in(&mut self, port: u8) -> u8 {
        match port {
            0 => 0x0F,
            1 => self.first_port,
            2 => self.second_port,
            3 => {
                let val = ((self.shift1 as u16) << 8) | self.shift0 as u16;
                ((val >> (8 - self.shift_offset)) & 0xFF) as u8
            }
            _ => panic!("Invalid port {:?} for IN", port),
        }
    }

    fn machine_out<M: MemoryMap>(&mut self, _: &mut Cpu<M>, port: u8, val: u8) {
        match port {
            2 => self.shift_offset = val & 0x7,
            3 => {
                self.third_port = val;
                if self.third_port != self.prev_third_port {
                }

                ()
            }
            4 => {
                self.shift0 = self.shift1;
                self.shift1 = val;
            }
            5 => {

                self.fifth_port = val;
                if self.fifth_port != self.prev_fifth_port {
                }
                ()
            }
            6 => {}
            _ => panic!("Invalid port {:?} for OUT", port),
        }
    }
}

impl SpaceInvadersIO {
    pub fn press(&mut self, key: Key, port: ControllerPort) {
        match port {
            ControllerPort::P1 => self.first_port |= key.bits(),
            ControllerPort::P2 => self.second_port |= key.bits(),
        }
    }

    pub fn release(&mut self, key: Key, port: ControllerPort) {
        match port {
            ControllerPort::P1 => self.first_port &= !key.bits(),
            ControllerPort::P2 => self.second_port &= !key.bits(),
        }
    }
}
