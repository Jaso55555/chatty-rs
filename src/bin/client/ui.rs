use std::io;
use std::io::Stdout;

use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture},
    execute,
    terminal::{disable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};

use tui::{backend::CrosstermBackend, Frame, Terminal};
use tui::backend::Backend;
use tui::layout::{Constraint, Direction, Layout};
use tui::widgets::{Block, Borders, List};
use common::message::{collect_messages, Message};

pub fn init() -> Terminal<CrosstermBackend<Stdout>> {
    let mut stdout = io::stdout();
    // Clear screen
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture).unwrap();

    let backend = CrosstermBackend::new(stdout);

    Terminal::new(backend).expect("Could not create terminal")
}

pub fn close(mut term: Terminal<CrosstermBackend<Stdout>>) {
    // restore terminal
    disable_raw_mode().unwrap();
    execute!(
        term.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    ).unwrap();
    term.show_cursor().unwrap();
}

// Draw code here
pub fn draw<B: Backend>(f: &mut Frame<B>, message_list: Vec<Message>) {

    let whole = Layout::default()
        .direction(Direction::Horizontal)
        .margin(1)
        .constraints(
            [
                Constraint::Percentage(20),
                Constraint::Percentage(80),
            ].as_ref()
        )
        .split(f.size());

    let chat = Layout::default()
        .direction(Direction::Vertical)
        .margin(0)
        .constraints(
            [
                Constraint::Min(5),
                Constraint::Length(2),
            ].as_ref()
        )
        .split(whole[1]);

    let channels = Block::default()
        .title(" Channels ")
        .borders(Borders::ALL);
    f.render_widget(channels, whole[0]);

    let messages = List::new(collect_messages(&message_list))
        .block(
            Block::default()
                .title("Messages")
                .borders(Borders::BOTTOM)
        );

    f.render_widget(messages, chat[0]);


    let _input_area = Block::default().inner(chat[1]);
}


