use std::env;
use std::thread;
use std::time::Duration;
use sysinfo::System;

mod sender;
mod size_calc;

fn main() {
    let mut args: Vec<String> = env::args().collect();
    let ip = args[1].as_mut_str();  

    loop {
        let mut sys = System::new_all();
        sys.refresh_all();
        
        let cpu = format!("{:.2}%", sys.global_cpu_info().cpu_usage());
        let name = format!("{}", size_calc::get_size(sys.free_swap()));
        let ram = format!("{}/{}", size_calc::get_size(sys.used_memory()), size_calc::get_size(sys.total_memory()));
        
        let message1 = format!("0§0{} {}", cpu, name);
        let message2 = format!("0§1{}", ram);
        
        if let Err(e) = sender::send(&message1, &message2, &ip) {
            eprintln!("Failed to send data: {}", e);
        }

        thread::sleep(Duration::from_secs(1));
    }
}
