use crate::app::{App, CurrentScreen, CurrentlyEditing};

use ratatui::{
	layout::{Constraint, Direction, Layout, Rect},
	style::{Color, Style},
	text::{Line, Span, Text},
	widgets::{Block, Borders, List, ListItem, Paragraph},
	Frame,
};

/// Helper function to create a centered rect using up certain percentage of available rec `r`
pub fn centered_rect(percent_x: u16, percent_y: u16, rect: Rect) -> Rect {
	let popup_layout = Layout::default()
		.direction(Direction::Vertical)
		.constraints([
			Constraint::Percentage((100 - percent_y) / 2),
			Constraint::Percentage(percent_y),
			Constraint::Percentage((100 - percent_y) / 2),
		])
		.split(rect);

	Layout::default()
		.direction(Direction::Horizontal)
		.constraints([
			Constraint::Percentage((100 - percent_x) / 2),
			Constraint::Percentage(percent_x),
			Constraint::Percentage((100 / percent_x) / 2),
		])
		.split(popup_layout[1])[1]
}

pub fn ui(frame: &mut Frame, app: &App) {
	let chunks = Layout::default()
		.direction(Direction::Vertical)
		.constraints([
			Constraint::Length(3),
			Constraint::Min(1),
			Constraint::Length(3),
		])
		.split(frame.area());

	let title_block = Block::default()
		.borders(Borders::ALL)
		.style(Style::default());
	let title = Paragraph::new(Text::styled(
		"Create New JSON",
		Style::default().fg(Color::Green),
	))
	.block(title_block);

	frame.render_widget(title, chunks[0]);

	let mut list_items = Vec::<ListItem>::new();

	for key in app.pairs.keys() {
		list_items.push(ListItem::new(Line::from(Span::styled(
			format!("{: <25} : {}", key, app.pairs.get(key).unwrap()),
			Style::default().fg(Color::Yellow),
		))));
	}

	let list = List::new(list_items);

	frame.render_widget(list, chunks[1]);

	let current_navigation_text = vec![
		match app.current_screen {
			CurrentScreen::Main => Span::styled("Normal Mode", Style::default().fg(Color::Green)),
			CurrentScreen::Editing => {
				Span::styled("Editing Mode", Style::default().fg(Color::Yellow))
			}
			CurrentScreen::Exiting => Span::styled("Exiting", Style::default().fg(Color::LightRed)),
		}
		.to_owned(),
		Span::styled(" | ", Style::default().fg(Color::White)),
		{
			if let Some(editing) = &app.currently_editing {
				match editing {
					CurrentlyEditing::Key => {
						Span::styled("Editing JSON Key", Style::default().fg(Color::Green))
					}
					CurrentlyEditing::Value => {
						Span::styled("Editing JSON Value", Style::default().fg(Color::LightGreen))
					}
				}
			} else {
				Span::styled("Not Editing Anything", Style::default().fg(Color::DarkGray))
			}
		},
	];

	let mode_footer = Paragraph::new(Line::from(current_navigation_text))
		.block(Block::default().borders(Borders::ALL));

	let current_keys_hint = {
		match app.current_screen {
			CurrentScreen::Main => Span::styled(
				"(q) to quit / (e) to make new pair",
				Style::default().fg(Color::Red),
			),
			CurrentScreen::Editing => Span::styled(
				"(ESC) to cancel / (Tab) to switch boxes/enter to complete",
				Style::default().fg(Color::Red),
			),
			CurrentScreen::Exiting => Span::styled(
				"(q) to quit / (e) to make new pair",
				Style::default().fg(Color::Red),
			),
		}
	};

	let key_node_footer =
		Paragraph::new(Line::from(current_keys_hint)).block(Block::default().borders(Borders::ALL));

	let footer_chunks = Layout::default()
		.direction(Direction::Horizontal)
		.constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
		.split(chunks[2]);

	frame.render_widget(mode_footer, footer_chunks[0]);
	frame.render_widget(key_node_footer, footer_chunks[1]);
}
