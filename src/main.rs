use std::io;
use std::fs::File;
use crossterm::*;
use ratatui::*;
use simplelog::*;

use ratatui::{
    widgets::{Block, Borders, Paragraph},
    layout::{Rect},
    style::{Style, Modifier, Color},
    text::{Span},
};

use crossterm::{
    event::{Event, KeyEvent, KeyEventKind, KeyCode},
    cursor::{SetCursorStyle},
    execute,
};

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

Log:
debug < info < warn < error
*/


const BORDER_COLOR: Color = Color::White;
const ACTIVE_TITLE_COLOR: Color = Color::Green;
const INACTIVE_TITLE_COLOR: Color = Color::Blue;


fn main() {
    WriteLogger::init(
        LevelFilter::Info,
        Config::default(),
        File::create("termite.log").unwrap(),
    ).unwrap();

    log::info!("Launching Termite...");

    let mut stdout = io::stdout();
    execute!(stdout, SetCursorStyle::SteadyUnderScore);

    let mut terminal = ratatui::init();                 // initialize ratatui
    let app_result = App::default().run(&mut terminal); // run application
    ratatui::restore();                                 // back to normal

    println!("Closing Termite...");
}

#[derive(Debug, Copy, Clone)]
enum InputState {
    Text,
    Path,
    Command,
}

impl InputState {
    fn filename(&self) -> &str {
        return match self {
            InputState::Text => "text_history.txt",
            InputState::Command => "command_history.txt",
            InputState::Path => "path_history.txt",
        }
    }
}

#[derive(Debug)]   // create default constructor and debug printing
struct App {
    exit: bool,             // should the program exit?
    state: InputState,         // how should it look?
    lines: u16,   // how many history lines are visible?
    width: u16,
    margin: u16,
    title: String,
    input: String,
    cursor_index: usize,
    bound: u16
}

impl Default for App {
    fn default() -> Self {
        let default_state = InputState::Text;
        let input = String::from(default_state.filename());
        let cursor_index = input.len();
        let bound = cursor_index;
        Self {
            exit: false,
            state: default_state,
            lines: 3,
            width: 10,
            margin: 3,
            title: "Command Line:".to_string(),
            input,
            cursor_index,
            bound: 10
        }
    }
}

impl App {

    fn get_autocomplete_pool(self) {
        match self.state {
            InputState::Path => {  }
            _ => {}
        };
    }

    fn open_file(self) {
        
    }


    fn exit(&mut self) {
        self.exit = true
    }

    fn submit(&mut self) {
        self.input.clear();
        self.cursor_index = 0
    }

    fn length_within_bounds(&self) -> bool {
        (self.input.len() as u16) < self.width - 2 * (self.margin + 2)
    }

    fn cursor_at_end(&self) -> bool {
        self.cursor_index == self.input.len()
    }

    fn left(&mut self) {
        if self.cursor_index > 0 {
            self.cursor_index -= 1
        }
    }

    fn right(&mut self) {
        if !self.cursor_at_end() {
            self.cursor_index += 1
        }
    }

    fn home(&mut self) {
        self.cursor_index = 0
    }

    fn end(&mut self) {
        self.cursor_index = self.input.len()
    }

    fn backspace(&mut self) {
        if self.cursor_index > 0 {
            if self.cursor_at_end() {
                self.input.pop();
            } else {
                self.input.remove(self.cursor_index-1);
            }
            self.cursor_index -= 1;
        }
    }

    fn delete(&mut self) {
        if !self.cursor_at_end() {
            self.input.remove(self.cursor_index);
        }
    }

    fn input(&mut self, c: char) {
        if self.length_within_bounds() {
            if self.cursor_at_end() {
                self.input.push(c);
            } else {
                self.input.insert(self.cursor_index, c);
            }
            self.cursor_index += 1;
        } else {

        }
    }

    /////////// RATATUI ////////////

    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        while !self.exit {                              // App loop
            //log::info!("{}", terminal.size().unwrap());
            terminal.draw(|frame| {
                self.width = frame.area().width;
                //log::info!("{}, {}", self.width, self.margin);
                self.draw(frame)})?;   // draw
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

        // Draw the input-bounding box and title
        frame.render_widget(
            Block::default()
                .borders(borders)
                .border_style(Style::default().fg(BORDER_COLOR))
                .title(format!(" {} ", self.title.clone()))
                .title_style(Style::default().fg(ACTIVE_TITLE_COLOR)),
            Rect::new(
                self.margin, 
                screen_area.height - self.lines, 
                screen_area.width - (2 * self.margin), 
                self.lines
            )
        );

        // Draw the textbox
        frame.render_widget(
            Paragraph::new(self.input.clone()),
            Rect::new(
                self.margin + 2,
                screen_area.height - 2,
                screen_area.width - (4 * self.margin) + 2,
                self.lines - 2
            )
        );
        frame.set_cursor_position((self.cursor_index as u16 + self.margin + 2, screen_area.height - 2));
        //log::info!("{}", self.width);
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
            KeyCode::Esc => self.exit(),
            KeyCode::Enter => self.submit(),
            KeyCode::Backspace => self.backspace(),
            KeyCode::Delete => self.delete(),
            KeyCode::Left => self.left(),
            KeyCode::Right => self.right(),
            KeyCode::Home => self.home(),
            KeyCode::End => self.end(),
            KeyCode::Char(c) => self.input(c),
            _ => {}
        };
    }
}
