use std::error::Error;
use std::io;

use json_editor::app::App;

use ratatui::{backend, crossterm::{
	event::{DisableMouseCapture, EnableMouseCapture},
	execute,
	terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
}, prelude::{Backend, CrosstermBackend}, Terminal};

fn run_app<B: Backend>(terminal: &mut Terminal<B>, app: &mut App) -> io::Result<bool> {
	todo!()
}

fn main() -> Result<(), Box<dyn Error>> {
	// Setup terminal
	enable_raw_mode()?;

	let mut stderr = io::stderr();

	execute!(stderr, EnterAlternateScreen, EnableMouseCapture)?;

	let backend = CrosstermBackend::new(stderr);
	let mut terminal = Terminal::new(backend)?;

	// Create app and run it
	let mut app = App::new();
	let res = run_app(&mut terminal, &mut app);

	// Restore terminal
	disable_raw_mode()?;
	execute!(terminal.backend_mut(), LeaveAlternateScreen, DisableMouseCapture)?;
	terminal.show_cursor()?;


	if let Ok(do_print) = res {
		if do_print {
			app.print_json()?;
		}
	} else if let Err(err) = res {
		eprintln!("{:?}", err);
	}

	Ok(())
}
