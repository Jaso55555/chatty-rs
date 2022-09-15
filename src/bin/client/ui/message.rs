use chrono::{DateTime, Local};
use tui::backend::Backend;
use tui::Frame;
use tui::layout::Rect;
use tui::widgets::{Block, Borders, Paragraph, Wrap};
use common::message::Message;
use tui::text::{Span, Spans};
use tui::style::{Color, Modifier, Style};

pub fn draw_messages<B: Backend>(f: &mut Frame<B>, message_list: &Vec<Message>, rect: Rect, scroll: u16) {
    let paragraph  = collect_messages(message_list, scroll);

    f.render_widget(paragraph, rect);
}

pub fn collect_messages<'a>(list: &Vec<Message>, scroll: u16) -> Paragraph<'a> {
    let mut lines = Vec::new();

    for item in list.iter() {
        let local_time: DateTime<Local> = DateTime::from(item.timestamp);

        lines.push(Spans::from(
            vec![
                Span::styled(
                    format!("{}", local_time.format("%d/%m/%Y %H:%M ")),
                    Style::default().add_modifier(Modifier::BOLD)
                ),
                Span::styled(
                    item.sender.clone(),
                    Style::default()
                        .add_modifier(Modifier::BOLD)
                        .fg(Color::Rgb(item.color[0], item.color[1], item.color[2]))
                ),
                Span::raw(" -> "),
                Span::raw(item.content.clone())
            ]
        ));
    }

    Paragraph::new(lines)
        .wrap(Wrap {
            trim: true
        })
        .scroll((scroll,0))
        .block(
            Block::default()
                .borders(Borders::BOTTOM)
        )
}
