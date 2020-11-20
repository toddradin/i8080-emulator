use std::io::{self, Write};

use i8080::cpu::Cpu;
use i8080::instruction::Instruction;
use i8080::machine::MachineIO;
use i8080::memory_bus::MemoryMap;

#[derive(Clone)]
struct TestMemory {
    pub memory: [u8; 0x10000],
}

impl TestMemory {
    fn new() -> Self {
        let mut buffer = [0; 0x10000];
        TestMemory::load_rom(&mut buffer);
        Self { memory: buffer }
    }
}

impl MemoryMap for TestMemory {
    fn load_rom(buffer: &mut [u8]) {
        let offset = 0x100;
        let rom = include_bytes!("../test-roms/8080EXM.COM");
        buffer[offset as usize..(rom.len() + offset as usize)].copy_from_slice(rom);
    }

    fn read(&mut self, addr: u16) -> u8 {
        self.memory[addr as usize]
    }

    fn read_slice(&mut self, addr: u16) -> &[u8] {
        &self.memory[addr as usize..]
    }

    fn write(&mut self, addr: u16, val: u8) {
        self.memory[addr as usize] = val;
    }
}

struct TestMachine;

impl MachineIO for TestMachine {
    fn machine_in(&mut self, _: u8) -> u8 {
        0
    }

    fn machine_out<M: MemoryMap>(&mut self, cpu: &mut Cpu<M>, port: u8, _: u8) {
        if port == 0 {
            println!("\n\nSignal to terminate the program has been received. Exiting.");
            std::process::exit(0);
        } else if port == 1 {
            if cpu.registers.c == 9 {
                let mut addr = cpu.registers.get_de() as usize;
                while cpu.memory.read(addr as u16) != b'$' {
                    print!("{}", cpu.memory.read(addr as u16) as char);
                    addr += 1;
                }
                io::stdout().flush().ok().expect("Could not flush stdout");
            } else if cpu.registers.c == 2 {
                print!("{}", cpu.registers.e as char);
                io::stdout().flush().ok().expect("Could not flush stdout");
            }
        }
    }
}

fn main() {
    let memory = TestMemory::new();
    let mut cpu = Cpu::new(memory);

    // The tests begin at 0x100 so advance pc to address
    cpu.pc = 0x100;

    // Map OUT 0,a to memory address 0x0. When machine_out() receives port 0,
    // the program will exit.
    cpu.memory.write(0x0, 0xD3);
    cpu.memory.write(0x1, 0x00);

    // Map OUT 1,a to memory address 0x5. When machine_out() receives port 1,
    // the program will output diagnostic or error messages from the test rom.
    cpu.memory.write(0x5, 0xD3);
    cpu.memory.write(0x6, 0x01);
    cpu.memory.write(0x7, 0xC9);

    let debug = false;

    loop {
        let instr = Instruction::from(cpu.memory.read_slice(cpu.pc));

        if debug {
            println!("{:?}", instr);
            println! {"pc: {:#x?}, sp: {:#x?},", cpu.pc, cpu.sp};
            println!("flags: {:#x?}", cpu.condition_codes.flags_to_psw());
            println!("{:#x?}\n", cpu.registers);
        }

        let (next_pc, _) = cpu.execute(&instr, &mut TestMachine);

        cpu.pc = next_pc;
    }
}
