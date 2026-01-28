use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use ratatui::{
    DefaultTerminal, Frame,
    widgets::ListState,
};
use crate::rules::rule::{Rule};

#[derive(Debug, Default, Clone, Copy)]
pub enum Screen{
    #[default]
    Main,
    Counter
}
#[derive(Debug, Default, Clone, Copy)]
pub enum Focus{
    #[default]
    RulesList,
    RuleManager,
}

#[derive(Debug, Default)]
pub struct App {
    running: bool,
    pub screen: Screen,
    pub focus: Focus,
    pub status_message: String,
    pub rules: Vec<Rule>,
    pub selection: ListState,
    pub selected_rule: Option<Rule>,
}

impl App {
    /// Construct a new instance of [`App`].
    pub fn new() -> Self {
        let mut state = ListState::default();
        state.select(Some(0));

        Self{
            selection: state,
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
        match (self.screen, self.focus, key.modifiers, key.code) {
            // Quit
            (_, _, _, KeyCode::Esc | KeyCode::Char('q'))
            | (_,_,KeyModifiers::CONTROL, KeyCode::Char('c'|'C')) => self.quit(),
            // Navigation
            (_, _, _, KeyCode::Char('1')) => self.screen = Screen::Main,
            (_, _, _, KeyCode::Char('2')) => self.screen = Screen::Counter,
            // Move arrows on ruleslist
            (Screen::Main, Focus::RulesList,_ ,KeyCode::Up) => self.rules_prev(),
            (Screen::Main, Focus::RulesList,_ ,KeyCode::Down) => self.rules_next(self.rules.len()),
            // Move arrows on rulemanager
            (Screen::Main, Focus::RuleManager,_ ,KeyCode::Up) => self.rules_prev(),
            (Screen::Main, Focus::RuleManager,_ ,KeyCode::Down) => self.rules_next(2),
            // Select rule
            (Screen::Main, Focus::RulesList,_ ,KeyCode::Enter) => self.select_rule(),
            // Counter btn
            // (Screen::Counter, _, _, KeyCode::Enter) if matches!(self.screen, Screen::Counter) => self.counter += 1,

            _ => {}
        };
    }

    // Helper for arrow navegation on menus
    fn rules_next(&mut self, menu_len: usize){
        let i = match self.selection.selected(){
            Some(i) => {
                if i>= menu_len-1{
                    i
                }else{
                    i+1
                }
            }
            None =>0
        };
        self.selection.select(Some(i));
    }

    fn rules_prev(&mut self){
        let i = match self.selection.selected(){
            Some(i) =>{
                if i==0{
                    0
                }else{
                    i-1
                }
            }
            None =>0
        };
        self.selection.select(Some(i));
    }

    fn select_rule(&mut self){
        self.selected_rule = self.
            selection
            .selected()
            .and_then(|i| self.rules.get(i).cloned())
    }

    /// Set running to false to quit the application.
    fn quit(&mut self) {
        self.running = false;
    }
}
