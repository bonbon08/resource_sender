use std::net::UdpSocket;

pub(crate) fn send(message1: &str, message2: &str, ip: &str) -> std::io::Result<()> {
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