mod tui;
mod watcher;
mod shell_commands;

use std::io;
use std::env;
use std::thread;
use std::sync::{Arc, RwLock, mpsc::{channel, Sender}};
use std::path::PathBuf;
use std::time::{Duration, Instant};

use crossbeam_channel::{unbounded, tick, select, Receiver};

use crate::watcher::FileWatcher;

use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::{
    prelude::*,
    symbols::border,
    widgets::{block::*, *},
};


fn main() -> io::Result<()> {
    // Get the container name from command line arguments
    let _args: Vec<String> = env::args().collect();

    if _args.len() != 2 {
        eprintln!("Usage: docker-manager <container-name>");
        std::process::exit(1);
    }

    let _container_name = _args[1].clone();
    let args: Vec<String> = env::args().collect();
    let mut terminal = tui::init()?;
    let app_result = App::new(_container_name).run(&mut terminal);
    tui::restore()?;
    app_result
}


pub struct App {
    selected_page: u8,
    exit: bool,
    watcher: Arc<RwLock<FileWatcher>>,
    tick_rate: Duration,
    // Add a receiver for tick events
    tick_rx: crossbeam_channel::Receiver<Instant>,
    input_rx: Receiver<Event>,
}

impl App {
    pub fn new(_container_name: String) -> App {
        let (tx, rx) = channel();
        let watcher: FileWatcher = FileWatcher::new(tx).expect("Failed to initialize FileWatcher");

        let watcher = Arc::new(RwLock::new(watcher));

        let watcher_clone = Arc::clone(&watcher);

        thread::spawn(move || {
            while let Ok(res) = rx.recv() {
                let mut watcher = watcher_clone.read().unwrap();
                logs.push("Watching for changes...".to_string());
                if let Err(e) = watcher.handle_event(res) {
                    eprintln!("Error handling events: {}", e);
                }
            }
        });

        let tick_rate = Duration::from_millis(100); // Adjust as needed
        let tick_rx = tick(tick_rate);

        // Create input event channel using crossbeam_channel
        let (input_tx, input_rx) = unbounded();

        // Spawn a thread to handle input events
        thread::spawn(move || {
            while let Ok(event) = event::read() {
                if input_tx.send(event).is_err() {
                    break;
                }
            }
        });

        App {
            selected_page: 1,
            exit: false,
            watcher,
            tick_rate,
            tick_rx,
            input_rx,
        }
    }
    /// runs the application's main loop until the user quits
    pub fn run(&mut self, terminal: &mut tui::Tui) -> io::Result<()> {
        while !self.exit {
            select! {
                recv(self.tick_rx) -> _ => {
                    terminal.draw(|frame| self.render_frame(frame))?;
                }
                recv(self.input_rx) -> event => {
                    if let Ok(event) = event {
                        self.handle_input_event(event)?;
                    }
                }
            }
        }
        Ok(())
    }

    fn render_frame(&self, frame: &mut Frame) {
        frame.render_widget(self, frame.size());
    }

    /// updates the application's state based on user input
    fn handle_input_event(&mut self, event: Event) -> io::Result<()> {
        match event {
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                self.handle_key_event(key_event);
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
                let instructions = Title::from(" Docker Manager ".bold());
                let title = Title::from(Line::from(vec![
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
                //let mut watcher_mut = self.watcher;
                //watcher_mut.handle_events().expect("Failed to handle file events!");
                let logs: Vec<ListItem> = {
                    let watcher = self.watcher.read().unwrap();
                    watcher.get_logs()
                        .iter()
                        .map(|log| ListItem::new(log.clone()))
                        .collect()
                };
                Widget::render(
                    List::new(logs)
                        .block(block)
                        .direction(ListDirection::TopToBottom),
                    area,
                    buf,
                );
                //Paragraph::new("Blabla")
                //    .block(block)
                //    .render(area, buf);
            }
            2 => {
                let instructions = Title::from(" Docker Manager ".bold());
                let title = Title::from(Line::from(vec![
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


                let watched_files: Vec<PathBuf> = {
                    self.watcher.read().unwrap().get_watched_files()
                        .iter()
                        .cloned()
                        .collect()
                };

                let items: Vec<ListItem> = watched_files
                    .iter()
                    .map(|file| ListItem::new(Span::styled(
                        file.display().to_string(),
                        Style::default().add_modifier(Modifier::BOLD),
                    )))
                    .collect();

                Widget::render(
                    List::new(items)
                        .block(block)
                        .direction(ListDirection::TopToBottom),
                    area,
                    buf,
                );
                //Paragraph::new("Blabla2")
                //    .block(block)
                //    .render(area, buf);
            }
            3 => {

                let instructions = Title::from(" Docker Manager ".bold());
                let title = Title::from(Line::from(vec![
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
