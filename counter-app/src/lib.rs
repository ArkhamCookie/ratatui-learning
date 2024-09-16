use std::{io, vec};

use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::{
	buffer::Buffer,
	layout::{Alignment, Rect},
	style::Stylize,
	symbols::border,
	text::{Line, Text},
	widgets::{
		block::{Position, Title},
		Block, Paragraph, Widget,
	},
	DefaultTerminal, Frame,
};

#[derive(Debug, Default)]
pub struct App {
	counter: u8,
	exit: bool,
}

impl App {
	/// Draws the render to terminal
	fn draw(&self, frame: &mut Frame) {
		frame.render_widget(self, frame.area());
	}

	fn exit(&mut self) {
		self.exit = true;
	}

	fn increment_counter(&mut self) {
		self.counter += 1;
	}

	fn decrement_counter(&mut self) {
		self.counter -= 1;
	}

	fn handle_key_event(&mut self, key_event: KeyEvent) {
		match key_event.code {
			KeyCode::Char('q') => self.exit(),
			KeyCode::Left => self.decrement_counter(),
			KeyCode::Right => self.increment_counter(),
			_ => {}
		}
	}

	/// Updates the applications's state based on user input
	fn handle_events(&mut self) -> io::Result<()> {
		match event::read()? {
			Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
				self.handle_key_event(key_event)
			}
			_ => {}
		};
		Ok(())
	}

	/// Runs the application's main loop (until user quits)
	pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
		while !self.exit {
			terminal.draw(|frame| self.draw(frame))?;
			self.handle_events()?;
		}
		Ok(())
	}
}

impl Widget for &App {
	fn render(self, area: Rect, buf: &mut Buffer) {
		let title = Title::from(" Counter App Tutorial ".bold());
		let instructions = Title::from(Line::from(vec![
			" Decrement ".into(),
			"<Left>".blue().bold(),
			" Increment ".into(),
			"<Right>".blue().bold(),
			" Quit ".into(),
			"<Q> ".blue().bold(),
		]));
		let block = Block::bordered()
			.title(title.alignment(Alignment::Center))
			.title(
				instructions
					.alignment(Alignment::Center)
					.position(Position::Bottom),
			)
			.border_set(border::THICK);
		let counter_text = Text::from(vec![Line::from(vec![
			"Value: ".into(),
			self.counter.to_string().yellow(),
		])]);

		Paragraph::new(counter_text)
			.centered()
			.block(block)
			.render(area, buf);
	}
}

#[cfg(test)]

mod tests {
	use super::*;

	#[test]
	fn handle_key_event() {
		let mut app = App::default();

		app.handle_key_event(KeyCode::Right.into());
		assert_eq!(app.counter, 1);

		app.handle_key_event(KeyCode::Left.into());
		assert_eq!(app.counter, 0);

		let mut app = App::default();

		app.handle_key_event(KeyCode::Char('q').into());
		assert!(app.exit);
	}
}
