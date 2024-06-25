mod tui;
mod watcher;
mod shell_commands;

use std::io;
use std::env;
use std::thread;
use std::sync::{Arc, Mutex};
use std::path::PathBuf;

use crate::watcher::FileWatcher;

use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::{
    prelude::*,
    symbols::border,
    widgets::{block::*, *},
};

fn main() -> io::Result<()> {
    // Get the container name from command line arguments
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("Usage: docker-manager <container-name>");
        std::process::exit(1);
    }

    let container_name = args[1].clone();
    let args: Vec<String> = env::args().collect();
    let mut terminal = tui::init()?;
    let app_result = App::new(container_name).run(&mut terminal);
    tui::restore()?;
    app_result
}

pub struct App {
    selected_page: u8,
    exit: bool,
    watcher: Arc<Mutex<FileWatcher>>,
}

impl App {
    pub fn new(container_name: String) -> App {
        let watcher = FileWatcher::new().expect("Failed to initialize FileWatcher");

        // Clone Arc for the thread
        //let watcher_clone = Arc::clone(&watcher);
        //thread::spawn(move || {
        //    let container_name = container_name.clone();
        //    let mut watcher = watcher_clone.lock().unwrap();
        //    watcher.handle_events(&container_name).expect("Failed to handle file events");
        //});

        App {
            selected_page: 1,
            exit: false,
            watcher,
        }
    }
    /// runs the application's main loop until the user quits
    pub fn run(&mut self, terminal: &mut tui::Tui) -> io::Result<()> {
        // Initialize the watcher
        while !self.exit {
            terminal.draw(|frame| self.render_frame(frame))?;
            self.handle_events()?;
        }
        Ok(())
    }

    fn render_frame(&self, frame: &mut Frame) {
        frame.render_widget(self, frame.size());
    }

    /// updates the application's state based on user input
    fn handle_events(&mut self) -> io::Result<()> {
        match event::read()? {
            // it's important to check that the event is a key press event as
            // crossterm also emits key release and repeat events on Windows.
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                self.handle_key_event(key_event)
            }
            _ => {}
        };
        Ok(())
    }

    fn handle_key_event(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Char('q') => self.exit(),
            KeyCode::Char('1') => self.change_page(1),
            KeyCode::Char('2') => self.change_page(2),
            KeyCode::Char('3') => self.change_page(3),
            _ => {}
        }
    }

    fn exit(&mut self) {
        self.exit = true;
    }

    fn change_page(&mut self, page: u8) {
        self.selected_page = page;
    }

}

impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        match self.selected_page {
            1 => {
                let title = Title::from(" Docker Manager ".bold());
                let instructions = Title::from(Line::from(vec![
                    " Console ".bold().into(),
                    "<1>".red().bold(),
                    " File Observer ".into(),
                    "<2>".blue().bold(),
                    " Container Manager ".into(),
                    "<3>".blue().bold(),
                    " Quit ".into(),
                    "<Q> ".blue().bold(),
                ]));
                let block = Block::default()
                    .title(title.alignment(Alignment::Center))
                    .title(
                        instructions
                            .alignment(Alignment::Center)
                            .position(Position::Bottom),
                    )
                    .borders(Borders::ALL)
                    .border_set(border::THICK);

                //let logs: Vec<ListItem> = {
                //    let mut watcher = self.watcher.lock().unwrap();
                //    watcher.get_logs()
                //        .lock()
                //        .unwrap()
                //        .iter()
                //        .map(|log| ListItem::new(log.clone()))
                //        .collect()
                //};

                //Widget::render(
                //    List::new(logs)
                //        .block(block)
                //        .direction(ListDirection::TopToBottom),
                //    area,
                //    buf,
                //);
                Paragraph::new("Blabla")
                    .block(block)
                    .render(area, buf);
            }
            2 => {
                let title = Title::from(" Docker Manager ".bold());
                let instructions = Title::from(Line::from(vec![
                    " Console ".into(),
                    "<1>".blue().bold(),
                    " File Observer ".bold().into(),
                    "<2>".red().bold(),
                    " Container Manager ".into(),
                    "<3>".blue().bold(),
                    " Quit ".into(),
                    "<Q> ".blue().bold(),
                ]));
                let block = Block::default()
                    .title(title.alignment(Alignment::Center))
                    .title(
                        instructions
                            .alignment(Alignment::Center)
                            .position(Position::Bottom),
                    )
                    .borders(Borders::ALL)
                    .border_set(border::THICK);

                //let watched_files: Vec<PathBuf> = {
                //    let watcher = self.watcher.lock().unwrap();
                //    watcher.get_watched_files()
                //        .iter()
                //        .cloned()
                //        .collect()
                //};

                //let items: Vec<ListItem> = watched_files
                //    .iter()
                //    .map(|file| ListItem::new(Span::styled(
                //        file.display().to_string(),
                //        Style::default().add_modifier(Modifier::BOLD),
                //    )))
                //    .collect();

                //Widget::render(
                //    List::new(items)
                //        .block(block)
                //        .direction(ListDirection::TopToBottom),
                //    area,
                //    buf,
                //);
                Paragraph::new("Blabla2")
                    .block(block)
                    .render(area, buf);
            }
            3 => {

                let title = Title::from(" Docker Manager ".bold());
                let instructions = Title::from(Line::from(vec![
                    " Console ".into(),
                    "<1>".blue().bold(),
                    " File Observer ".into(),
                    "<2>".blue().bold(),
                    " Container Manager ".bold().into(),
                    "<3>".red().bold(),
                    " Quit ".into(),
                    "<Q> ".blue().bold(),
                ]));
                let block = Block::default()
                    .title(title.alignment(Alignment::Center))
                    .title(
                        instructions
                            .alignment(Alignment::Center)
                            .position(Position::Bottom),
                    )
                    .borders(Borders::ALL)
                    .border_set(border::THICK);

                Paragraph::new("Blabla3")
                    .block(block)
                    .render(area, buf);
            }
            _ => {}
        }
    }
}

//#[cfg(test)]
//mod tests {
//    use super::*;
//
//    #[test]
//    fn render() {
//        let app = App::default();
//        let mut buf = Buffer::empty(Rect::new(0, 0, 50, 4));
//
//        app.render(buf.area, &mut buf);
//
//        let mut expected = Buffer::with_lines(vec![
//            "┏━━━━━━━━━━━━━ Docker Manager ━━━━━━━━━━━━━┓",
//            "┃                    Value: 0                    ┃",
//            "┃                                                ┃",
//            "┗━ Console <1> File Observer <2> Container Manager <3> Quit <Q> ━━┛",
//        ]);
//        let title_style = Style::new().bold();
//        let counter_style = Style::new().yellow();
//        let key_style = Style::new().blue().bold();
//        expected.set_style(Rect::new(14, 0, 22, 1), title_style);
//        expected.set_style(Rect::new(28, 1, 1, 1), counter_style);
//        expected.set_style(Rect::new(13, 3, 6, 1), key_style);
//        expected.set_style(Rect::new(30, 3, 7, 1), key_style);
//        expected.set_style(Rect::new(43, 3, 4, 1), key_style);
//
//        // note ratatui also has an assert_buffer_eq! macro that can be used to
//        // compare buffers and display the differences in a more readable way
//        assert_eq!(buf, expected);
//    }
//
//    #[test]
//    fn handle_key_event() -> io::Result<()> {
//        let mut app = App::default();
//        app.handle_key_event(KeyCode::Char('1').into());
//        assert_eq!(app.counter, 1);
//
//        app.handle_key_event(KeyCode::Char('2').into());
//        assert_eq!(app.counter, 0);
//
//        let mut app = App::default();
//        app.handle_key_event(KeyCode::Char('q').into());
//        assert_eq!(app.exit, true);
//
//        Ok(())
//    }
//}
