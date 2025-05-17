use rust_udp_utility::UdpHandler;

fn main() {
    let mut udp = UdpHandler::new("UdpHandler", true);

    udp.open_auto_address(1000);
    udp.open_auto_address(2000);

    udp.set_send_period(500);
    udp.set_destination("192.168.11.65:64201");

    loop {
        udp.send("Hello".as_bytes());
    }
}
