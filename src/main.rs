mod shell_commands;
mod watcher;
mod ui;
mod utils;

use std::env;
use watcher::Watcher;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, KeyCode, KeyEvent},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::backend::{CrosstermBackend};
use ratatui::Terminal;
use ratatui::widgets::{Block, Borders, List, ListItem};
use ratatui::layout::{Layout, Constraint, Direction};
use ratatui::style::{Style, Color, Modifier};
use std::io::{stdout};
use std::path::PathBuf;
use std::time::{Duration};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Get the container name from command line arguments
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("Usage: docker-manager <container-name>");
        std::process::exit(1);
    }

    let container_name = &args[1];

    // Initialize the watcher
    let mut watcher = Watcher::new().expect("Failed to initialize inotify");

    // Setup terminal
    enable_raw_mode()?;
    execute!(stdout(), EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout());
    let mut terminal = Terminal::new(backend)?;

    let mut selected_page = 0;
    let watched_files: Vec<PathBuf> = watcher.get_watched_files().into_iter().cloned().collect();

    // Event loop
    loop {

        //match watcher.handle_events(&container_name) {
        //    Ok(_) => {

        //    },
        //    Err(err) => {
        //    },
        //}
        terminal.draw(|f| {
            let size = f.size();
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Percentage(100)].as_ref())
                .split(size);

            match selected_page {
                0 => {
 
                let mut log_items: Vec<ListItem> = watcher
                        .get_logs()
                        .iter()
                        .map(|log| ListItem::new(log.clone()))
                        .collect();

                    //watcher.handle_events(&container_name);
                    // Handle file changes and handle result from watcher
                    //match watcher.handle_events(&container_name) {
                    //    Ok(_) => {

                    //    },
                    //    Err(err) => {
                    //        log_items.push(ListItem::new(format!("Error handling events: {}", err)));
                    //    },
                    //}
                    // Handle file changes        
                    //if let Err(e) = watcher.handle_events(&container_name) {
                    //    log_items.push(ListItem::new(format!("Error handling events: {}", e)));
                    //}

                    let list = List::new(log_items)
                        .block(Block::default().title("Logs").borders(Borders::ALL));

                    f.render_widget(list, chunks[0]);
                }
                1 => {
                    let items: Vec<ListItem> = watched_files
                        .iter()
                        .map(|file| ListItem::new(file.display().to_string()))
                        .collect();

                    let list = List::new(items)
                        .block(Block::default().title("File Explorer").borders(Borders::ALL))
                        .highlight_style(
                            Style::default()
                                .bg(Color::LightGreen)
                                .fg(Color::Black)
                                .add_modifier(Modifier::BOLD),
                        )
                        .highlight_symbol("> ");

                    f.render_widget(list, chunks[0]);
                }
                _ => {}
            }
        })?;

        if event::poll(Duration::from_millis(100))? {
            if let event::Event::Key(KeyEvent { code, .. }) = event::read()? {
                match code {
                    KeyCode::Char('q') => {
                        disable_raw_mode()?;
                        execute!(terminal.backend_mut(), LeaveAlternateScreen, DisableMouseCapture)?;
                        terminal.show_cursor()?;
                        break;
                    }
                    KeyCode::Char('1') => {
                        selected_page = 0;
                    }
                    KeyCode::Char('2') => {
                        selected_page = 1;
                    }
                    _ => {}
                }
            } else if let event::Event::Resize(_, _) = event::read()? {
                terminal.autoresize()?;
            }
        }
    }

    Ok(())
}
