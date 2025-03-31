use std::io;
use crossterm::*;
use ratatui::*;

use ratatui::{
    widgets::{Block, Borders, Paragraph},
    layout::{Rect},
    style::{Style, Modifier, Color},
    text::{Span},
};

use crossterm::event::{Event, KeyEvent, KeyEventKind, KeyCode};

/* these are just for reference of other things i might want to import
use crossterm::event::{self, KeyEvent};
use ratatui::{
    buffer::Buffer,
    style::Stylize,
    symbols::border,
    text::{Line, Text},
    widgets::{Widget},
    DefaultTerminal, Frame,
};
*/


const BORDER_COLOR: Color = Color::White;
const ACTIVE_TITLE_COLOR: Color = Color::Green;
const INACTIVE_TITLE_COLOR: Color = Color::Blue;


fn main() {
    println!("Initializing ratatui...");
    let mut terminal = ratatui::init();                 // initialize ratatui
    let app_result = App::default().run(&mut terminal); // run application
    ratatui::restore();                                 // back to normal
    println!("Returning from ratatui...");
}

//enum State {
//    Expanded, // like a typical terminal with some amount of history lines above the input line
//    Minimal,  // a single input line
//}

#[derive(Debug)]   // create default constructor and debug printing
struct App {
    exit: bool,             // should the program exit?
    //state: State,         // how should it look?
    lines: u16,   // how many history lines are visible?
    width: u16,
    margin: u16,
    title: String,
    text_buffer: String,
}

impl Default for App {
    fn default() -> Self {
        Self {
            exit: false,
            lines: 3,
            width: 10,
            margin: 3,
            title: "Command Line:".to_string(),
            text_buffer: "".to_string(),
        }
    }
}

impl App {

    fn exit(&mut self) {
        self.exit = true
    }

    /////////// RATATUI ////////////

    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        while !self.exit {                              // App loop
            terminal.draw(|frame| self.draw(frame))?;   // draw
            self.handle_events()?;                      // handle key presses
        }
        Ok(())
    }

    fn draw(&self, frame: &mut Frame) {
        let screen_area: Rect = frame.area();

        let borders = match &self.lines {
            1 => Borders::NONE,
            2 => Borders::TOP,
            _ => Borders::ALL,
        };

        frame.render_widget(
            Block::default()
                .borders(borders)
                .border_style(Style::default().fg(BORDER_COLOR))
                .title(format!(" {} ", self.title.clone()))
                .title_style(Style::default().fg(ACTIVE_TITLE_COLOR).remove_modifier(Modifier::BOLD)),
            Rect::new(
                self.margin, 
                screen_area.height - self.lines, 
                screen_area.width - (2 * self.margin), 
                self.lines
            )
        );
    }

    fn handle_events(&mut self) -> io::Result<()> {
        match event::read()? {
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                self.handle_keypress(key_event)
            }
            _ => {}
        }
        Ok(())
    }

    fn handle_keypress(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Char('q') => self.exit(),
            //KeyCode::Char(c) => 
            _ => {}
        };
    }
}
