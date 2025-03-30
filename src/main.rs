use std::io;
use crossterm::*;
use ratatui::*;

use ratatui::{
    widgets::{Block, Borders, Paragraph},
    layout::{Rect},
};

use crossterm::event::{Event, KeyEvent, KeyEventKind};

/* these are just for reference of other things i might want to import
use crossterm::event::{self, KeyCode, KeyEvent};
use ratatui::{
    buffer::Buffer,
    style::Stylize,
    symbols::border,
    text::{Line, Text},
    widgets::{Widget},
    DefaultTerminal, Frame,
};
*/




fn main() {
    println!("Initializing ratatui...");
    let mut terminal = ratatui::init();                 // initialize ratatui
    let app_result = App::default().run(&mut terminal); // run application
    ratatui::restore();                                 // back to normal
    println!("Returning from ratatui...");
}

#[derive(Default, Debug)]   // create default constructor and debug printing
struct App {
    exit: bool, // should the program exit?
}

impl App {
    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        while !self.exit {                              // App loop
            terminal.draw(|frame| self.draw(frame))?;   // draw
            self.handle_events()?;                      // handle key presses
        }
        Ok(())
    }

    fn draw(&self, frame: &mut Frame) {
        frame.render_widget(Block::default().borders(Borders::ALL), Rect::new(0, 0, 5, 5)); // create a small box with borders
    }

    fn handle_events(&mut self) -> io::Result<()> {
        match event::read()? {
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => { // in enum this is saying, if we have a Key, save as key_event
                self.exit = true // terminate whenever a key is pressed
            }
            _ => {} // nothing happens otherwise
        }
        Ok(()) // all good!
    }
}
