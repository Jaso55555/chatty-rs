mod net;
mod ui;

use chrono::Utc;
use crossterm::event::Event;
use std::io;
use std::io::Stdout;
use std::time::Duration;
use tui::backend::CrosstermBackend;
use tui::Terminal;
use tui_textarea::{CursorMove, Input, Key};

use crate::net::NetCodeState;
use crate::ui::choice::Choice;
use crate::ui::crash;
use common::client_config::ClientConfig;
use common::logs;
use common::message::Message;
use common::net::active::ActivePacket;
use net::NetCode;
use ui::UIStorage;

fn main() -> Result<(), io::Error> {
    logs::client_log_init();

    let mut term = ui::init();

    let (config, _new_config) = ClientConfig::load();

    let mut net = match NetCode::init(&config) {
        Ok(net) => net,
        Err(e) => {
            crash(Some(term), e);
            return Ok(());
        }
    };

    let mut message_list = vec![Message {
        sender: "[CLIENT]".to_string(),
        content: "Connected to server".to_string(),
        timestamp: Utc::now(),
        color: [255, 247, 0],
    }];

    let mut ui = UIStorage::new();
    let mut scroll = 0;

    loop {
        match net.behave(&config) {
            Some(Ok(msg)) => match msg {
                ActivePacket::Message(msg) | ActivePacket::SystemMessage(msg) => {
                    message_list.push(msg)
                }
                ActivePacket::Shutdown { reason } => {
                    crash(Some(term), reason);
                    return Ok(());
                }
            },
            Some(Err(e)) => {
                crash(Some(term), e);
                return Ok(());
            }
            _ => {}
        }

        term.draw(|f| ui.draw(f, &message_list, scroll, net.check_state().into()))?;

        if let Ok(true) = crossterm::event::poll(Duration::from_millis(100)) {
            match crossterm::event::read()?.into() {
                Input { key: Key::Esc, .. } => {
                    crash(
                        Some(term),
                        match net.shutdown("User disconnect") {
                            Ok(v) => v,
                            Err(e) => e.to_string(),
                        },
                    );

                    break;
                }
                Input { key: Key::Tab, .. } => {
                    ui.choice.toggle_focus();
                }
                Input {
                    key: Key::Enter, ..
                } => {
                    // Make sure we're connected first
                    if *net.check_state() == NetCodeState::Active {
                        // Send message
                        if ui.text_area.lines()[0].len() > 0 {
                            let msg = Message {
                                sender: config.username.clone(),
                                content: ui.text_area.lines()[0].clone(),
                                timestamp: Utc::now(),
                                color: config.user_color,
                            };

                            let packet = ActivePacket::Message(msg);

                            net.send_packet(&packet).expect("Could not send message");

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
    }

    Ok(())
}