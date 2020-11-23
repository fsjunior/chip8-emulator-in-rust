/* Memory Mappings */
/*
    0x000-0x1FF - Chip 8 interpreter (contains font set in emu)
    0x050-0x0A0 - Used for the built in 4x5 pixel font set (0-F)
    0x200-0xFFF - Program ROM and work RAM

    Source: http://www.multigesture.net/articles/how-to-write-an-emulator-chip-8-interpreter/
*/

pub struct CPU {
    pub opcode: u16,
    pub memory: Box<[u8; 4096]>,
    pub v: Box<[u8; 16]>,
    pub i: u16,
    pub pc: u16,
}

impl CPU {
    pub fn new() -> CPU {
        CPU {
            opcode: 0,
            memory: Box::new([0u8; 4096]),
            v: Box::new([0u8; 16]),
            i: 0,
            pc: 0,
        }
    }
}
