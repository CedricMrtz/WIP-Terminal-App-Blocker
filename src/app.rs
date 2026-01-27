use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use ratatui::{
    DefaultTerminal, Frame,
    widgets::ListState,
};
use crate::rules::rule::{Rule};

#[derive(Debug, Default)]
pub enum Screen{
    #[default]
    Main,
    Counter
}

#[derive(Debug, Default)]
pub struct App {
    running: bool,
    pub screen: Screen,
    pub counter: u32,
    pub status_message: String,
    pub rules: Vec<Rule>,
    pub ruleslist_state: ListState,
    pub selected_rule: Option<Rule>,
}

impl App {
    /// Construct a new instance of [`App`].
    pub fn new() -> Self {
        let mut state = ListState::default();
        state.select(Some(0));

        Self{
            ruleslist_state: state,
            ..Default::default()
        }
    }



    /// Run the application's main loop.
    pub fn run(mut self, mut terminal: DefaultTerminal) -> color_eyre::Result<()> {
        self.running = true;
        while self.running {
            terminal.draw(|frame| self.render(frame))?;
            self.handle_crossterm_events()?;
        }
        Ok(())
    }

    ///Render
    fn render(&mut self, frame: &mut Frame) {
        crate::ui::render(frame, self);
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
            // Move arrows on ruleslist
            (_, KeyCode::Up) if matches!(self.screen, Screen::Main) => self.rules_prev(),
            (_, KeyCode::Down) if matches!(self.screen, Screen::Main) => self.rules_next(),
            // Select rule
            (_, KeyCode::Enter) if matches!(self.screen, Screen::Main) => self.select_rule(),
            // Counter btn
            (_, KeyCode::Enter) if matches!(self.screen, Screen::Counter) => self.counter += 1,

            _ => {}
        };
    }

    // Helper for arrow navegation on ruleslist
    fn rules_next(&mut self){
        let i = match self.ruleslist_state.selected(){
            Some(i) => {
                if i>= self.rules.len()-1{
                    i
                }else{
                    i+1
                }
            }
            None =>0
        };
        self.ruleslist_state.select(Some(i));
    }

    fn rules_prev(&mut self){
        let i = match self.ruleslist_state.selected(){
            Some(i) =>{
                if i==0{
                    0
                }else{
                    i-1
                }
            }
            None =>0
        };
        self.ruleslist_state.select(Some(i));
    }

    fn select_rule(&mut self){
        self.selected_rule = self.
            ruleslist_state
            .selected()
            .and_then(|i| self.rules.get(i).cloned())
    }

    /// Set running to false to quit the application.
    fn quit(&mut self) {
        self.running = false;
    }
}
