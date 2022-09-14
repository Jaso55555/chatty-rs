mod config;
mod ui;

use std::thread::sleep;
use std::time::Duration;
use std::io;

use common::message::Message;

fn main() -> Result<(), io::Error> {
    let _config = config::Config::load();

    let mut term = ui::init();

    let message_list = vec![
        Message {
            sender: "Guy 1".to_string(),
            content: "My balls itch".to_string()
        },
        Message {
            sender: "Guy 2".to_string(),
            content: "Damn bro".to_string()
        }
    ];

    term.draw(
        |f| {
            ui::draw(f, message_list)
        }
    ).expect("Could not draw UI");

    sleep(Duration::from_secs(5));

    ui::close(term);

    Ok(())

    // let mut socket = TcpStream::connect("127.0.0.1:5678").expect("Could not connect to server");
    // loop {
    //     match Message::try_read(&mut socket) {
    //         None => {}
    //         Some(msg) => {println!("{}", msg)}
    //     }
    //
    //     sleep(Duration::from_millis(100))
    // }
}