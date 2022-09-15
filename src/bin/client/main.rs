mod config;
mod ui;
mod net;

use std::io;
use chrono::Utc;
use tui_textarea::{CursorMove, Input, Key};

use common::message::Message;
use crate::net::NetCode;
use crate::ui::UIStorage;

fn main() -> Result<(), io::Error> {
    let config = config::Config::load();

    let mut term = ui::init();
    let mut ui = UIStorage::new();
    let mut scroll = 0;

    let mut net = match NetCode::init() {
        Ok(net) => net,
        Err(e) => panic!("Network error: {}", e)
    };

    let mut message_list = Vec::new();

    loop {
        match net.check_for_messages() {
            Some(msg) => message_list.push(msg),
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

    ui::close(term);

    Ok(())
}