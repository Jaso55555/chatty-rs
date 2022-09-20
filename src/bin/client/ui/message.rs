use chrono::{DateTime, Local};
use common::message::Message;
use tui::backend::Backend;
use tui::layout::Rect;
use tui::style::{Color, Modifier, Style};
use tui::text::{Span, Spans};
use tui::widgets::{Block, Borders, Paragraph, Wrap};
use tui::Frame;

pub fn draw_messages<B: Backend>(
    f: &mut Frame<B>,
    message_list: &Vec<Message>,
    rect: Rect,
    scroll: u16,
    state: bool,
) {
    let paragraph = collect_messages(message_list, scroll, state);

    f.render_widget(paragraph, rect);
}

pub fn collect_messages<'a>(list: &Vec<Message>, scroll: u16, state: bool) -> Paragraph<'a> {
    let mut lines = Vec::new();

    for item in list.iter() {
        let local_time: DateTime<Local> = DateTime::from(item.timestamp);

        lines.push(Spans::from(vec![
            Span::styled(
                format!("{}", local_time.format("%d/%m/%Y %H:%M ")),
                Style::default().add_modifier(Modifier::BOLD),
            ),
            Span::styled(
                item.sender.clone(),
                Style::default().add_modifier(Modifier::BOLD).fg(Color::Rgb(
                    item.color[0],
                    item.color[1],
                    item.color[2],
                )),
            ),
            Span::raw(" -> "),
            Span::raw(item.content.clone()),
        ]));
    }

    let state_color = match state {
        true => Color::White,
        false => Color::LightYellow,
    };

    Paragraph::new(lines)
        .wrap(Wrap { trim: true })
        .scroll((scroll, 0))
        .block(
            Block::default()
                .borders(Borders::BOTTOM)
                .border_style(Style::default().fg(state_color)),
        )
}
