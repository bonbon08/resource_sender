use std::net::UdpSocket;
use std::thread;
use std::time::Duration;
use std::env;
use sysinfo::System;

fn get_size(bytes: u64) -> String {
    let mut size = bytes as f64;
    let factor = 1024.0;
    let units = ["B", "K", "M", "G", "T", "P"];
    
    for unit in units.iter() {
        if size < factor {
            return format!("{:.2}{}", size, unit);
        }
        size /= factor;
    }
    format!("{:.2}P", size)
}

fn send(message1: &str, message2: &str, ip: &str) -> std::io::Result<()> {
    let socket = UdpSocket::bind("0.0.0.0:0")?;
    let port = 80;
    let addr = format!("{}:{}", ip, port);
    
    let blank1 = "0§0                ";
    let blank2 = "0§1                ";
    
    socket.send_to(blank1.as_bytes(), &addr)?;
    socket.send_to(message1.as_bytes(), &addr)?;
    socket.send_to(blank2.as_bytes(), &addr)?;
    socket.send_to(message2.as_bytes(), &addr)?;
    
    Ok(())
}

fn main() {
    let mut args: Vec<String> = env::args().collect();
    let ip = args[1].as_mut_str();  

    loop {
        let mut sys = System::new_all();
        sys.refresh_all();
        
        let cpu = format!("{:.2}%", sys.global_cpu_info().cpu_usage());
        let name = format!("{}", get_size(sys.free_swap()));
        let ram = format!("{}/{}", get_size(sys.used_memory()), get_size(sys.total_memory()));
        
        let message1 = format!("0§0{} {}", cpu, name);
        let message2 = format!("0§1{}", ram);
        
        if let Err(e) = send(&message1, &message2, &ip) {
            eprintln!("Failed to send data: {}", e);
        }

        thread::sleep(Duration::from_secs(1));
    }
}
