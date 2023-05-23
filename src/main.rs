use std::fs::File;
use std::io::Read;
use std::{
    thread,
    time::{Duration, Instant},
};

mod cpu;
mod gameboy;
mod instruction;

fn main() {
    let mut gameboy = gameboy::create_gameboy();
    let mut bootloader_file = File::open("bootloader.bin").unwrap();
    bootloader_file.read(&mut gameboy.ram).unwrap();
    println!("{:?}", gameboy.ram);

    // loop at 4.2 MHz
    loop {
        let start_time = Instant::now();
        // 16.6 ms as nanoseconds
        let frame_time = Duration::new(0, 16600000);
        // cycles are in t-cycles
        // reference: http://www.codeslinger.co.uk/pages/projects/gameboy/opcodes.html
        let cycles_per_frame = (cpu::CPU_FREQUENCY / 60.0) as i64;
        let mut emulated_cycles: i64 = 0;

        while emulated_cycles < cycles_per_frame {
            emulated_cycles += gameboy::step_cpu(&mut gameboy);
        }

        let elapsed_time = start_time.elapsed();
        if !(elapsed_time > frame_time) {
            let remaining_time = frame_time - elapsed_time;
            thread::sleep(remaining_time);
        }
    }
}
