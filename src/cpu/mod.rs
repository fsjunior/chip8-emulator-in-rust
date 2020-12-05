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
    pub v: [u8; 16],
    pub i: u16,
    pub pc: u16,
    //Stack
    pub stack: [u16; 16],
    pub sp: u16,
    // Graphics
    pub gfx: Box<[[bool; 64]; 32]>,
    //Sound
    pub delay_timer: u8,
    pub sound_timer: u8,
    //Keyboard
    pub keyboard: [u8; 16],
}

impl CPU {
    pub fn new() -> CPU {
        CPU {
            opcode: 0,
            memory: Box::new([0; 4096]),
            v: [0; 16],
            i: 0,
            pc: 0,
            stack: [0; 16],
            sp: 0,
            gfx: Box::new([[false; 64]; 32]),
            delay_timer: 0,
            sound_timer: 0,
            keyboard: [0; 16],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let cpu = CPU::new();
        assert_eq!(cpu.opcode, 0);
        assert_eq!(cpu.memory, Box::new([0; 4096]));
        assert_eq!(cpu.v, [0; 16]);
        assert_eq!(cpu.i, 0);
        assert_eq!(cpu.pc, 0);
        assert_eq!(cpu.stack, [0; 16]);
        assert_eq!(cpu.sp, 0);
        assert_eq!(cpu.gfx, Box::new([[false; 64]; 32]));
        assert_eq!(cpu.delay_timer, 0);
        assert_eq!(cpu.sound_timer, 0);
        assert_eq!(cpu.keyboard, [0; 16]);
    }
}
