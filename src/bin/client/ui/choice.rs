use tui::backend::Backend;
use tui::layout::{Constraint, Direction, Layout, Margin, Rect};
use tui::style::{Modifier, Style};
use tui::text::{Span, Spans, Text};
use tui::widgets::{Clear, List, ListItem, ListState, Paragraph};
use tui::Frame;

pub struct Choice<'a> {
    focus: bool,
    question: Paragraph<'a>,
    options: List<'a>,
    state: ListState,
    rect: Rect,
}

impl<'a> Choice<'a> {
    pub fn new<Q: Into<Text<'a>>>(question: Q, options: Vec<String>) -> Self {
        let mut _options = Vec::new();

        for item in options {
            _options.push(ListItem::new(Spans::from(Span::raw::<String>(item))))
        }

        Self {
            focus: false,
            question: Paragraph::new(question.into())
                .style(Style::default().add_modifier(Modifier::BOLD)),
            options: List::new(_options),
            state: Default::default(),
            rect: Default::default(),
        }
    }

    pub fn focused(&self) -> bool {
        self.focus.clone()
    }

    pub fn toggle_focus(&mut self) {
        self.focus ^= true;
    }

    pub fn draw<B: Backend>(&mut self, f: &mut Frame<B>, rect: Rect) {
        let layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints(vec![Constraint::Percentage(20), Constraint::Percentage(80)])
            .split(rect);

        f.render_widget(
            Clear,
            rect.inner(&Margin {
                vertical: 1,
                horizontal: 1,
            }),
        );

        f.render_widget(self.question.clone(), layout[0]);

        f.render_stateful_widget(self.options.clone(), self.rect, &mut self.state)
    }
}
