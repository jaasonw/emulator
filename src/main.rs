mod cpu;

fn main() {
    let cpu = cpu::cpu::CPU::new();
    println!("{}", cpu.a);
}
