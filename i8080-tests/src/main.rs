use std::io::{self, Write};

use i8080::cpu::Cpu;
use i8080::instruction::Instruction;
use i8080::machine::MachineIO;
use i8080::memory_bus::MemoryMap;

struct TestMachine<'a> {
    cpu: &'a mut Cpu<TestMemory>,
}

impl<'a> MachineIO for TestMachine<'a> {
    fn machine_in(&mut self, _: u8) -> u8 {
        0
    }

    fn machine_out(&mut self, port: u8, _: u8) {
        if port == 0 {
            println!("\n\nSignal to terminate the program has been received. Exiting.");
            std::process::exit(0);
        } else if port == 1 {
            if self.cpu.registers.c == 9 {
                let mut addr = self.cpu.registers.get_de() as usize;
                while self.cpu.memory.read(addr as u16) != b'$' {
                    print!("{}", self.cpu.memory.read(addr as u16) as char);
                    addr += 1;
                }
                io::stdout().flush().ok().expect("Could not flush stdout");
            } else if self.cpu.registers.c == 2 {
                print!("{}", self.cpu.registers.e as char);
                io::stdout().flush().ok().expect("Could not flush stdout");
            }
        }
    }
}

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
        let rom = include_bytes!("../test-roms/CPUTEST.COM");
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
    let mut i = 0;
    const STEP_CYCLES: u16 = 16_667;
    loop {
        let mut cyc = 0;
        
        while cyc < STEP_CYCLES {
            let instr = Instruction::from(cpu.memory.read_slice(cpu.pc));
            let (next_pc, cycles) = cpu.execute(
                &instr,
                &mut TestMachine {
                    cpu: &mut cpu.clone(),
                },
            );
            cpu.pc = next_pc;
            cyc += cycles as u16;
            if debug {
                println!("{:?} {:?}", i, instr);
                println! {"pc: {:#x?}, sp: {:#x?},", cpu.pc, cpu.sp};
                println!("cycles: {}", cycles);
                println!("{:#x?}", cpu.condition_codes);
                println!("{:#x?}\n", cpu.registers);
            }
            i += 1;
        }
        std::thread::sleep(std::time::Duration::from_micros(16));

    }
}
