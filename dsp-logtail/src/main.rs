use chrono::Local;
use memmap2::Mmap;
use std::time::Duration;


mod sharespace;
mod util;

use std::env;

fn hexdump(buffer: &[u8]) {
    for (i, chunk) in buffer.chunks(16).enumerate() {
        print!("{:08x}: ", i * 16);
        for byte in chunk {
            print!("{:02x} ", byte);
        }
        for _ in 0..(16 - chunk.len()) {
            print!("   ");
        }
        print!(" ");
        for &byte in chunk {
            if byte >= 32 && byte <= 126 {
                print!("{}", byte as char);
            } else {
                print!(".");
            }
        }
        println!();
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let dump_mode = args.iter().any(|arg| arg == "-dump");

    let log_buffer = sharespace::mmap_log_buffer();

    if dump_mode {
        hexdump(&log_buffer);
        return;
    }
    println!("Log buffer mapped at: {:p}", log_buffer.as_ptr());

    let mut read_ptr = 4;

    loop {
        let write_ptr = u32::from_le_bytes(log_buffer[0..4].try_into().unwrap()) as usize;

        if write_ptr == read_ptr {
            std::thread::sleep(Duration::from_millis(100));
            continue;
        }

        let process_buffer = |buffer: &[u8]| {
            for message in buffer.split(|&b| b == 0) {
                if !message.is_empty() {
                    let timestamp = Local::now().format("%Y-%m-%d %H:%M:%S");
                    println!("{}: {}", timestamp, String::from_utf8_lossy(message).trim_end());
                }
            }
        };

        if write_ptr > read_ptr {
            process_buffer(&log_buffer[read_ptr..write_ptr]);
        } else { // write_ptr < read_ptr, wrapped around
            process_buffer(&log_buffer[read_ptr..]);
            process_buffer(&log_buffer[4..write_ptr]);
        }
        
        read_ptr = write_ptr;

        std::thread::sleep(Duration::from_millis(100));
    }
}