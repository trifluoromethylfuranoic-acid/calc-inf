extern crate iced;

use std::fmt;
use std::fmt::{Display, write};
use std::sync::Arc;

use iced::widget::text_editor::{Action, Edit};
use iced::widget::{button, column, row, text, text_editor};
use iced::{Application, Element, Size, application, window};

fn main() {
	application("Calculator", update, view)
		.window(window::Settings {
			size: Size {
				width: 350.,
				height: 500.,
			},
			position: Default::default(),
			visible: true,
			resizable: false,
			decorations: true,
			level: Default::default(),
			icon: None,
			exit_on_close_request: true,
			..Default::default()
		})
		.run()
		.unwrap();
}

fn update(state: &mut State, message: Message) {
	match message {
		Message::ButtonPressed(but) => match but {
			CalcButton::Number(n) => state
				.input
				.perform(Action::Edit(Edit::Paste(Arc::new(n.to_string())))),
			CalcButton::Clear => {}
			CalcButton::Eval => {}
		},
		Message::Edit(action) => state.input.perform(action),
	}
}

fn view(state: &State) -> Element<Message> {
	column![
		text_editor(&state.input)
			.on_action(Message::Edit)
			.height(200),
		row![
			calc_button(CalcButton::Number(7)),
			calc_button(CalcButton::Number(8)),
			calc_button(CalcButton::Number(9))
		],
		row![
			calc_button(CalcButton::Number(4)),
			calc_button(CalcButton::Number(5)),
			calc_button(CalcButton::Number(6))
		],
		row![
			calc_button(CalcButton::Number(1)),
			calc_button(CalcButton::Number(2)),
			calc_button(CalcButton::Number(3))
		],
	]
	.into()
}

#[derive(Default)]
struct State {
	input: text_editor::Content,
}

#[derive(Debug, Clone)]
enum Message {
	ButtonPressed(CalcButton),
	Edit(text_editor::Action),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum CalcButton {
	Number(u32),
	Clear,
	Eval,
}

impl Display for CalcButton {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
		match self {
			CalcButton::Number(n) => {
				write!(f, "{n}")
			}
			CalcButton::Clear => {
				write!(f, "C")
			}
			CalcButton::Eval => {
				write!(f, "=")
			}
		}
	}
}

fn calc_button(but: CalcButton) -> iced::widget::Button<'static, Message> {
	button(text(but.to_string()))
		.on_press(Message::ButtonPressed(but))
		.width(60)
		.height(70)
}
