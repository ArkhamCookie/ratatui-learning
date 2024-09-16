use std::io;

use counter_app::App;

fn main() -> io::Result<()> {
	let mut terminal = ratatui::init();
	let app_result = App::default().run(&mut terminal);

	ratatui::restore();
	app_result
}
