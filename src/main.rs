use std::io;
use std::io::*;
use std::fs::{File, create_dir_all};
use std::path::{Path, PathBuf};
use std::fmt::Display;
use std::thread;
use crossterm::*;
use ratatui::*;
use simplelog::*;
use dirs::home_dir;


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

const DATA_DIRECTORY: &str = ".local/share/termite/";


fn main() {
    WriteLogger::init(
        LevelFilter::Info,
        Config::default(),
        File::create("termite.log").expect("failed to create log file"),
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
    name: String,
    state: InputState,         // how should it look?
    loaded_pool_index: usize,
    pool_file: PathBuf,
    pool: Vec<String>,
    pool_index: usize,
    lines: u16,   // how many history lines are visible?
    width: u16,
    margin: u16,
    title: String,
    input: String,
    ignore_history_input: String,
    cursor_index: usize
}

impl Default for App {
    fn default() -> Self {
        let default_state = InputState::Text;
        let mut pool_file = home_dir().expect("Failed to find home directory");
        pool_file.push(DATA_DIRECTORY);
        pool_file.push(default_state.filename());
        log::info!("Reading from {}", pool_file.display().to_string());
        let input = String::from(DATA_DIRECTORY.to_string() + default_state.filename());
        let ignore_history_input = String::from(DATA_DIRECTORY.to_string() + default_state.filename());
        let cursor_index = input.len();
        Self {
            exit: false,
            name: "Termite".to_string(),
            state: default_state,
            loaded_pool_index: 0,
            pool_file,
            pool: Vec::new(),
            pool_index: 0,
            lines: 3,
            width: 10,
            margin: 3,
            title: "Command Line:".to_string(),
            input,
            ignore_history_input,
            cursor_index
        }
    }
}

impl App {

    fn exit(&mut self) {
        //self.save_temp_pool();
        if self.input == "" {
            self.exit = true;
        } else {
            self.wipe();
        }
    }

    fn submit(&mut self) {
        if self.input.len() > 0 {
            self.pool.push(self.input.clone());
            self.append_to_pool();
            self.wipe();
        }
    }

    fn wipe(&mut self) {
        self.pool_index = self.pool.len();
        self.input.clear();
        self.ignore_history_input = self.input.clone();
        self.correct_cursor();
    }

    fn correct_cursor(&mut self) {
        self.cursor_index = self.input.len()
    }

    fn length_within_bounds(&self) -> bool {
        (self.input.len() as u16) < self.width - 2 * (self.margin + 2)
    }

    fn cursor_at_end(&self) -> bool {
        self.cursor_index == self.input.len()
    }

    fn down(&mut self) {
        if self.pool_index == self.pool.len() - 1 {
            self.pool_index += 1;
            self.input = self.ignore_history_input.clone();
        } else if self.pool_index < self.pool.len() - 1 {
            self.pool_index += 1;
            self.input = self.pool[self.pool_index].clone();
        }
        self.correct_cursor();
    }

    fn up(&mut self) {
        if self.pool_index > 0 {
            self.pool_index -= 1;
            self.input = self.pool[self.pool_index].clone();
        }
        self.correct_cursor();
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
        self.correct_cursor();
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
            self.ignore_history_input = self.input.clone();
            self.pool_index = self.pool.len();
        }
    }

    fn append_to_pool(&self) {
        let path = self.pool_file.clone();
        let input = self.input.clone();

        thread::spawn(move || {
            let _ = writeln!(
                File::options().append(true).open(path).unwrap(),
                "{}",
                input
            );
        });
    }

    // We already use a thread to append each command, this might not be necesary
    //fn save_temp_pool(&self) {
    //    let mut file = File::options().append(true).open(self.pool_file.as_path()).unwrap();
    //    for entry in &self.pool[self.loaded_pool_index..self.pool.len()] {
    //        writeln!(&mut file, "{}", entry).unwrap();
    //    }
    //}

    fn change_state(&mut self, new_state: InputState) {
        // first, append all new pool entries
        //self.save_temp_pool();
        self.state = new_state;
        self.pool_file.pop();
        let _ = create_dir_all(self.pool_file.as_path());
        self.pool_file.push(self.state.filename());
        self.read_history_file();
    }

    fn read_history_file(&mut self) {
        if !self.pool_file.exists() {
            let _ = File::create_new(self.pool_file.as_path());
        }
        let file = File::open(self.pool_file.as_path()).expect("Failed to open input file");
        let reader = BufReader::new(file);
        for line in reader.lines() {
            let line = line.expect("Failed to read line!");  // each line is a Result<String, io::Error>
            log::info!("Line: {}", line);
            self.pool.push(line);
            self.pool_index += 1;
        }
        self.loaded_pool_index = self.pool.len();
    }

    /////////// RATATUI ////////////

    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        self.read_history_file();
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

    fn init(&mut self) {
        self.change_state(InputState::Text);
        self.read_history_file();
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
            KeyCode::Up => self.up(),
            KeyCode::Down => self.down(),
            KeyCode::Left => self.left(),
            KeyCode::Right => self.right(),
            KeyCode::Home => self.home(),
            KeyCode::End => self.end(),
            KeyCode::Char(c) => self.input(c),
            _ => {}
        };
    }
}
