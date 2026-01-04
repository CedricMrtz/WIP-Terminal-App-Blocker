use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use ratatui::{
    DefaultTerminal, Frame,
    style::Stylize,
    text::Line,
    widgets::{Block, Paragraph},
};

#[derive(Debug, Default)]
pub enum Screen{
    #[default]
    Main,
    Counter
}

#[derive(Debug, Default)]
pub struct App {
    running: bool,
    screen: Screen,
    counter: u32,
}

impl App {
    /// Construct a new instance of [`App`].
    pub fn new() -> Self {
        Self::default()
    }

    /// Run the application's main loop.
    pub fn run(mut self, mut terminal: DefaultTerminal, text: &str) -> color_eyre::Result<()> {
        self.running = true;
        while self.running {
            terminal.draw(|frame| self.render(frame, text))?;
            self.handle_crossterm_events()?;
        }
        Ok(())
    }

    ///widgets
    fn render(&mut self, frame: &mut Frame, message: &str) {
        match self.screen{
            Screen::Main => self.render_main(frame, message),
            Screen::Counter => self.render_counter(frame),
        }
    }
    fn render_main(&self, frame: &mut Frame, message: &str){
        let title = Line::from("Ratatui Simple Template")
            .bold()
            .blue()
            .centered();
        let text = message.to_string();
        frame.render_widget(
            Paragraph::new(text)
                .block(Block::bordered().title(title))
                .centered(),
            frame.area(),
        )
    }
    fn render_counter(&self, frame: &mut Frame){
        let text = format!(
            "Counter Screen\n\nPressed: {}\n\n[Enter] Increment\n[1] Back",
            self.counter
        );

        frame.render_widget(
            Paragraph::new(text)
                .block(Block::bordered().title("Counter")),
            frame.area(),
        );
    }

    /// Reads the crossterm events and updates the state of [`App`].
    ///
    /// If your application needs to perform work in between handling events, you can use the
    /// [`event::poll`] function to check if there are any events available with a timeout.
    fn handle_crossterm_events(&mut self) -> color_eyre::Result<()> {
        match event::read()? {
            // it's important to check KeyEventKind::Press to avoid handling key release events
            Event::Key(key) if key.kind == KeyEventKind::Press => self.on_key_event(key),
            Event::Mouse(_) => {}
            Event::Resize(_, _) => {}
            _ => {}
        }
        Ok(())
    }

    /// Handles the key events and updates the state of [`App`].
    fn on_key_event(&mut self, key: KeyEvent) {
        match (key.modifiers, key.code) {
            // Quit
            (_, KeyCode::Esc | KeyCode::Char('q'))
            | (KeyModifiers::CONTROL, KeyCode::Char('c') | KeyCode::Char('C')) => self.quit(),
            // Navigation
            (_, KeyCode::Char('1')) => self.screen = Screen::Main,
            (_, KeyCode::Char('2')) => self.screen = Screen::Counter,
            // Counter btn
            (_, KeyCode::Enter) if matches!(self.screen, Screen::Counter) => self.counter += 1,

            _ => {}
        }
    }

    /// Set running to false to quit the application.
    fn quit(&mut self) {
        self.running = false;
    }
}
