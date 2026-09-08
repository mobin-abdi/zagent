use pulldown_cmark::{Event, Parser, Tag, TagEnd};
use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout},
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Wrap},
};

use crate::agent::message::Role;

use super::app::App;

pub fn draw(frame: &mut Frame, app: &mut App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(1), Constraint::Length(3)])
        .split(frame.area());

    draw_chat(frame, app, chunks[0]);
    draw_input(frame, app, chunks[1]);
}

fn draw_chat(frame: &mut Frame, app: &mut App, area: ratatui::layout::Rect) {
    let mut lines = Vec::new();

    for message in &app.agent.state.messages {
        match message.role {
            Role::System => {}

            Role::User => {
                lines.push(Line::from(""));
                lines.push(Line::from(Span::styled(
                    "You",
                    Style::default().add_modifier(Modifier::BOLD),
                )));

                if let Some(content) = &message.content {
                    lines.push(Line::from(content.clone()));
                }
            }

            Role::Assistant => {
                lines.push(Line::from(""));
                lines.push(Line::from(Span::styled(
                    "ZAgent",
                    Style::default().add_modifier(Modifier::BOLD),
                )));

                if let Some(content) = &message.content {
                    lines.extend(render_markdown(content));
                }
            }

            Role::Tool => {
                if let Some(name) = &message.name {
                    lines.push(Line::from(Span::styled(
                        format!("⚙ {}", name),
                        Style::default().add_modifier(Modifier::DIM),
                    )));
                }
            }
        }
    }

    if app.loading {
        lines.push(Line::from(""));
        lines.push(Line::from(Span::styled(
            "ZAgent is thinking...",
            Style::default().add_modifier(Modifier::ITALIC),
        )));
    }

    let inner_height = area.height.saturating_sub(2);

    let content_height = lines.len() as u16;

    let max_scroll = content_height.saturating_sub(inner_height);

    if app.auto_scroll {
        app.scroll = max_scroll;
    }

    if app.scroll > max_scroll {
        app.scroll = max_scroll;
    }

    let paragraph = Paragraph::new(lines)
        .block(Block::default().title(" ZAgent ").borders(Borders::ALL))
        .wrap(Wrap { trim: false })
        .scroll((app.scroll, 0));

    frame.render_widget(paragraph, area);
}

fn draw_input(frame: &mut Frame, app: &App, area: ratatui::layout::Rect) {
    let paragraph = Paragraph::new(app.input.as_str())
        .block(Block::default().title(" Message ").borders(Borders::ALL))
        .wrap(Wrap { trim: false });

    frame.render_widget(paragraph, area);

    frame.set_cursor_position((area.x + app.input.len() as u16 + 1, area.y + 1));
}

fn render_markdown(markdown: &str) -> Vec<Line<'static>> {
    let parser = Parser::new(markdown);

    let mut lines = Vec::new();
    let mut current = Vec::new();

    let mut bold = false;
    let mut italic = false;
    let mut code = false;
    let mut list_depth = 0usize;

    for event in parser {
        match event {
            Event::Start(Tag::Heading { .. }) => {
                if !current.is_empty() {
                    lines.push(Line::from(std::mem::take(&mut current)));
                }
            }

            Event::End(TagEnd::Heading(_)) => {
                if !current.is_empty() {
                    lines.push(Line::from(std::mem::take(&mut current)));
                }
            }

            Event::Start(Tag::Strong) => {
                bold = true;
            }

            Event::End(TagEnd::Strong) => {
                bold = false;
            }

            Event::Start(Tag::Emphasis) => {
                italic = true;
            }

            Event::End(TagEnd::Emphasis) => {
                italic = false;
            }

            Event::Start(Tag::CodeBlock(_)) => {
                code = true;

                if !current.is_empty() {
                    lines.push(Line::from(std::mem::take(&mut current)));
                }
            }

            Event::End(TagEnd::CodeBlock) => {
                code = false;

                if !current.is_empty() {
                    lines.push(Line::from(std::mem::take(&mut current)));
                }
            }

            Event::Start(Tag::List(_)) => {
                list_depth += 1;
            }

            Event::End(TagEnd::List(_)) => {
                list_depth = list_depth.saturating_sub(1);
            }

            Event::Start(Tag::Item) => {
                current.push(Span::raw("• "));
            }

            Event::End(TagEnd::Item) => {
                if !current.is_empty() {
                    lines.push(Line::from(std::mem::take(&mut current)));
                }
            }

            Event::Text(text) => {
                let mut style = Style::default();

                if bold {
                    style = style.add_modifier(Modifier::BOLD);
                }

                if italic {
                    style = style.add_modifier(Modifier::ITALIC);
                }

                if code {
                    style = style.add_modifier(Modifier::DIM);
                }

                current.push(Span::styled(text.to_string(), style));
            }

            Event::Code(text) => {
                current.push(Span::styled(
                    text.to_string(),
                    Style::default().add_modifier(Modifier::DIM),
                ));
            }

            Event::SoftBreak | Event::HardBreak => {
                if !current.is_empty() {
                    lines.push(Line::from(std::mem::take(&mut current)));
                }
            }

            Event::Rule => {
                lines.push(Line::from("────────────────────────"));
            }

            _ => {}
        }
    }

    if !current.is_empty() {
        lines.push(Line::from(current));
    }

    lines
}

fn content_height(lines: &[Line]) -> u16 {
    lines.len() as u16
}
