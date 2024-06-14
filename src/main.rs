// Import necessary libraries and modules
// use std::fs::File;
// use std::io::{self, Read};

// Define the structure to represent the 8086 registers
struct CPU {
    // 16-bit general purpose registers
    // ax: u16, // AX register
    // bx: u16, // BX register
    // cx: u16, // CX register
    // dx: u16, // DX register

    // Segment registers
    // si: u16, // Source Index
    // di: u16, // Destination Index
    // bp: u16, // Base Pointer
    // sp: u16, // Stack Pointer

    // Memory array
    // memory: [u8; 1024], // Example memory size
}

impl CPU {
    // Function to create a new CPU instance
    fn new() -> CPU {
        // Initialize registers and memory
        // CPU { 
        //     ax: 0, bx: 0, cx: 0, dx: 0, 
        //     si: 0, di: 0, bp: 0, sp: 0, 
        //     memory: [0; 1024]
        // }
    }

    // Function to reset the CPU
    fn reset(&mut self) {
        // Reset all registers and memory
        // self.ax = 0;
        // self.bx = 0;
        // self.cx = 0;
        // self.dx = 0;
        // self.si = 0;
        // self.di = 0;
        // self.bp = 0;
        // self.sp = 0;
        // self.memory = [0; 1024];
    }
}

// Define the structure to represent memory
struct Memory {
    // memory: [u8; 1024], // Example memory size
}

impl Memory {
    // Function to create a new Memory instance
    fn new() -> Memory {
        // Initialize memory
        // Memory { memory: [0; 1024] }
    }

    // Function to read from memory
    fn read(&self, address: usize, size: usize) -> &[u8] {
        // Return a slice of memory
        // &self.memory[address..address + size]
    }

    // Function to write to memory
    fn write(&mut self, address: usize, data: &[u8]) {
        // Write data to memory at the specified address
        // for (i, &byte) in data.iter().enumerate() {
        //     self.memory[address + i] = byte;
        // }
    }
}

// Define the structure to represent instructions
struct Instruction {
    // opcode: u8, // Opcode for the instruction
    // w: bool, // W bit for instruction size (8-bit or 16-bit)
    // reg: u8, // Register operand
    // rm: u8, // Register/memory operand
}

impl Instruction {
    // Function to decode an instruction from a binary stream
    fn decode(bytes: &[u8]) -> Instruction {
        // Decode the instruction fields from the byte stream
        // Instruction { 
        //     opcode: bytes[0], 
        //     w: (bytes[1] & 0b10000000) != 0, 
        //     reg: (bytes[1] >> 3) & 0b111, 
        //     rm: bytes[1] & 0b111 
        // }
    }

    // Function to execute the decoded instruction
    fn execute(&self, cpu: &mut CPU) {
        // Execute the instruction based on the decoded fields
        // For now, we'll just handle a basic MOV instruction
        // if self.opcode == 0b10001000 {
        //     if self.w {
        //         // 16-bit move
        //         let src = match self.reg {
        //             0 => cpu.ax,
        //             1 => cpu.cx,
        //             2 => cpu.dx,
        //             3 => cpu.bx,
        //             4 => cpu.sp,
        //             5 => cpu.bp,
        //             6 => cpu.si,
        //             7 => cpu.di,
        //             _ => 0,
        //         };
        //         match self.rm {
        //             0 => cpu.ax = src,
        //             1 => cpu.cx = src,
        //             2 => cpu.dx = src,
        //             3 => cpu.bx = src,
        //             4 => cpu.sp = src,
        //             5 => cpu.bp = src,
        //             6 => cpu.si = src,
        //             7 => cpu.di = src,
        //             _ => (),
        //         }
        //     } else {
        //         // 8-bit move (AL/AX)
        //         let src = (cpu.ax & 0xFF) as u8;
        //         match self.rm {
        //             0 => cpu.ax = (cpu.ax & 0xFF00) | src as u16,
        //             _ => (),
        //         }
        //     }
        // }
    }
}

// Function to load binary instruction files into memory
fn load_binary(file_path: &str, memory: &mut Memory) -> io::Result<()> {
    // Open the binary file
    // let mut file = File::open(file_path)?;
    
    // Read the file contents into memory
    // file.read_to_end(&mut memory.memory)?;
    
    // Return success
    // Ok(())
}

// Unit test to validate the instruction decoding process
#[test]
fn test_instruction_decode() {
    // Initialize test data
    // let bytes: [u8; 2] = [0b10110000, 0b11000000]; // Example instruction
    // let instruction = Instruction::decode(&bytes);
    
    // Validate the decoded instruction fields
    // assert_eq!(instruction.opcode, 0b10110000);
    // assert_eq!(instruction.w, true);
    // assert_eq!(instruction.reg, 0b011);
    // assert_eq!(instruction.rm, 0b000);
}

// Unit test to validate the CPU execution process
#[test]
fn test_cpu_execution() {
    // Initialize the CPU and memory
    // let mut cpu = CPU::new();
    // let mut memory = Memory::new();
    
    // Load test binary file into memory
    // load_binary("test_files/listing_37.bin", &mut memory).unwrap();
    
    // Decode and execute instructions
    // let instruction = Instruction::decode(&memory.read(0, 2));
    // instruction.execute(&mut cpu);
    
    // Validate the CPU state after execution
    // assert_eq!(cpu.ax, 0x1234); // Example expected result
}

fn main() {
    // Initialize the CPU and memory
    // let mut cpu = CPU::new();
    // let mut memory = Memory::new();
    
    // Load binary file into memory
    // loader::load_binary("intel_8086_mov.txt", &mut memory).unwrap();
    
    // Decode and execute instructions
    // let instruction = instruction::Instruction::decode(&memory.read(0, 2));
    // instruction.execute(&mut cpu);
    
    // Print CPU state for debugging
    // println!("{:?}", cpu);
}
