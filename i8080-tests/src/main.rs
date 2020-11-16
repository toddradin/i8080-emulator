use i8080::cpu::Cpu;
use i8080::instruction::Instruction;
use i8080::machine::MachineIO;

struct TestMachine<'a> {
    cpu: &'a mut Cpu,
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
                while self.cpu.memory[addr] != b'$' {
                    print!("{}", self.cpu.memory[addr] as char);
                    addr += 1;
                }
            } else if self.cpu.registers.c == 2 {
                print!("{}", self.cpu.registers.e as char);
            }
        }
    }
}

fn main() -> Result<(), std::io::Error> {
    let mut cpu = Cpu::new();

    let offset = 0x100;
    let buffer = include_bytes!("../test-roms/TST8080.COM");
    cpu.memory[offset as usize..(buffer.len() + offset as usize)].copy_from_slice(buffer);

    // The tests begin at 0x100 so advance pc to address
    cpu.pc = 0x100;

    // Map OUT 0,a to memory address 0x0. When machine_out() receives port 0,
    // the program will exit.
    cpu.memory[0x0] = 0xD3;
    cpu.memory[0x1] = 0x00;

    // Map OUT 1,a to memory address 0x5. When machine_out() receives port 1,
    // the program will output diagnostic or error messages from the test rom.
    cpu.memory[0x5] = 0xD3;
    cpu.memory[0x6] = 0x01;
    cpu.memory[0x7] = 0xC9;

    let debug = false;
    let mut i = 0;
    while cpu.pc < cpu.memory.len() as u16 {
        let instr = Instruction::from(&cpu.memory[cpu.pc as usize..]);
        let (next_pc, cycles) = cpu.execute(
            &instr,
            &mut TestMachine {
                cpu: &mut cpu.clone(),
            },
        );
        cpu.pc = next_pc;

        if debug {
            println!("{:?} {:?}", i, instr);
            println! {"pc: {:#x?}, sp: {:#x?},", cpu.pc, cpu.sp};
            println!("cycles: {}", cycles);
            println!("{:#x?}", cpu.condition_codes);
            println!("{:#x?}\n", cpu.registers);
        }
        i += 1;
    }

    Ok(())
}
