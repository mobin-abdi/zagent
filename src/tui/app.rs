use std::io::{self, Stdout};

use anyhow::Result;
use crossterm::{
    event::{
        self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEvent, KeyModifiers,
        MouseEvent, MouseEventKind, DisableBracketedPaste, EnableBracketedPaste,
    },
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use ratatui::{Terminal, backend::CrosstermBackend};

use crate::agent::{agent::Agent, message::Message};

use super::ui;

pub struct App {
    pub agent: Agent,
    pub input: String,
    pub should_quit: bool,
    pub loading: bool,

    // Scroll
    pub scroll: u16,
    pub auto_scroll: bool,
}

impl App {
    pub fn new(agent: Agent) -> Self {
        Self {
            agent,
            input: String::new(),
            should_quit: false,
            loading: false,
            scroll: 0,
            auto_scroll: true,
        }
    }

    pub async fn submit(&mut self) -> Result<()> {
        let input = self.input.trim().to_string();

        if input.is_empty() || self.loading {
            return Ok(());
        }

        self.input.clear();

        self.agent.add_message(Message::user(input));

        self.auto_scroll = true;

        self.loading = true;

        let result = self.agent.run().await;

        self.loading = false;

        self.auto_scroll = true;

        if let Err(error) = result {
            self.agent
                .add_message(Message::assistant(format!("**Error:** `{error}`")));
        }

        Ok(())
    }
}

pub async fn run(agent: Agent) -> Result<()> {
    enable_raw_mode()?;

    let mut stdout = io::stdout();

    execute!(stdout, EnterAlternateScreen, EnableMouseCapture, EnableBracketedPaste)?;

    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let result = run_app(&mut terminal, agent).await;

    // Cleanup
    disable_raw_mode()?;

    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture,
        DisableBracketedPaste,
    )?;

    terminal.show_cursor()?;

    result
}

async fn run_app(terminal: &mut Terminal<CrosstermBackend<Stdout>>, agent: Agent) -> Result<()> {
    let mut app = App::new(agent);

    loop {
        terminal.draw(|frame| {
            ui::draw(frame, &mut app);
        })?;

        if event::poll(std::time::Duration::from_millis(100))? {
            match event::read()? {
                Event::Key(key) => {
                    handle_key_event(&mut app, key).await?;
                }

                Event::Paste(text) => {
                    if !app.loading {
                        app.input.push_str(&text);
                        app.auto_scroll = true;
                    }
                }

                Event::Mouse(mouse) => {
                    handle_mouse_event(&mut app, mouse);
                }

                _ => {}
            }
        }

        if app.should_quit {
            break;
        }
    }

    Ok(())
}

async fn handle_key_event(app: &mut App, key: KeyEvent) -> Result<()> {
    if key.modifiers.contains(KeyModifiers::CONTROL) && key.code == KeyCode::Char('c') {
        app.should_quit = true;
        return Ok(());
    }

    if app.loading {
        return Ok(());
    }

    match key.code {
        KeyCode::Esc => {
            app.should_quit = true;
        }

        KeyCode::Enter => {
            app.submit().await?;
        }

        KeyCode::Backspace => {
            app.input.pop();
        }

        KeyCode::Char(c) => {
            app.input.push(c);
        }

        KeyCode::Up => {
            app.scroll = app.scroll.saturating_sub(1);
            app.auto_scroll = false;
        }

        KeyCode::Down => {
            app.scroll = app.scroll.saturating_add(1);
        }

        KeyCode::PageUp => {
            app.scroll = app.scroll.saturating_sub(10);
            app.auto_scroll = false;
        }

        KeyCode::PageDown => {
            app.scroll = app.scroll.saturating_add(10);
        }

        _ => {}
    }

    Ok(())
}

fn handle_mouse_event(app: &mut App, mouse: MouseEvent) {
    match mouse.kind {
        MouseEventKind::ScrollUp => {
            app.scroll = app.scroll.saturating_sub(3);
            app.auto_scroll = false;
        }

        MouseEventKind::ScrollDown => {
            app.scroll = app.scroll.saturating_add(3);
        }

        _ => {}
    }
}
