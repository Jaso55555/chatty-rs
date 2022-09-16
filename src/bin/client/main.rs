mod ui;
mod net;

use std::fs::File;
use std::io;
use chrono::Utc;
use tui_textarea::{CursorMove, Input, Key};

use common::message::Message;
use common::client_config::ClientConfig;
use net::NetCode;
use ui::UIStorage;
use crate::net::NetCodeState;
use crate::ui::{close, crash};
use simplelog;


fn main() -> Result<(), io::Error> {
    simplelog::WriteLogger::init(
        simplelog::LevelFilter::Info,
        simplelog::Config::default(),
        File::create(
            format!("logs\\client-{}.txt", Utc::now().format("%d-%m-%Y-%H-%M"))
        ).expect("Could not create log")
    ).expect("Could not init logger");
    let config = ClientConfig::load();

    let mut net = match NetCode::init(&config) {
        Ok(net) => net,
        Err(e) => {
            crash(None, e);
            return Ok(())
        }
    };

    let mut message_list = vec![
        Message {
            sender: "[CLIENT]".to_string(),
            content: "Connected to server".to_string(),
            timestamp: Utc::now(),
            color: [255, 247, 0]
        }
    ];

    let mut term = ui::init();
    let mut ui = UIStorage::new();
    let mut scroll = 0;

    loop {
        match net.behave(&config) {
            Some(Ok(msg)) => message_list.push(msg),
            Some(Err(e)) => {
                crash(Some(term), e);
                return Ok(())
            }
            _ => {}
        }

        term.draw(|f| {
            ui.draw(f, &message_list, scroll)
        })?;

        match crossterm::event::read()?.into() {
            Input { key: Key::Esc, .. } => {
                break
            }
            Input { key: Key::Enter, .. } => {
                // Make sure we're connected first
                if *net.check_state() == NetCodeState::Active {
                    // Send message
                    if ui.text_area.lines()[0].len() > 0 {
                        let msg = Message {
                            sender: config.username.clone(),
                            content: ui.text_area.lines()[0].clone(),
                            timestamp: Utc::now(),
                            color: config.user_color
                        };

                        net.send_message(&msg).expect("Could not send message");
                        message_list.push(msg);


                        ui.text_area.move_cursor(CursorMove::Head);
                        ui.text_area.delete_line_by_end();
                    }
                }
            }
            Input { key: Key::Up, .. } => {
                if scroll > 0 {
                    scroll -= 1;
                }
            }
            Input { key: Key::Down, .. } => {
                if scroll < (message_list.len() - 1) as u16 {
                    scroll += 1
                }
            }

            input => {
                if ui.text_area.input(input) {
                    // If textbox is changed, do stuff here
                }
            }
        }


    }

    close(term);

    Ok(())
}