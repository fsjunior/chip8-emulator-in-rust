mod cpu;

fn main() {
    let cpu = cpu::CPU::new();
    println!("{:?}", cpu.gfx);
}
