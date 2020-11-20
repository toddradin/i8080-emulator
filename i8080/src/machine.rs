use crate::cpu::Cpu;
use crate::memory_bus::MemoryMap;

pub trait MachineIO {
    fn machine_in(&mut self, port: u8) -> u8;

    fn machine_out<M: MemoryMap>(&mut self, cpu: &mut Cpu<M>, port: u8, val: u8);
}
