mod lexer;
mod parser;
mod eval;

extern crate iced;

use std::fmt;
use std::fmt::{Display, write};
use std::sync::Arc;

use iced::widget::text_editor::{Action, Edit};
use iced::widget::{button, column, row, text, text_editor, text_input};
use iced::{Application, Element, Size, application, window};
use crate::lexer::Lexer;
use crate::parser::Parser;

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
			CalcButton::Clear => {
				state.input.perform(Action::SelectAll);
				state.input.perform(Action::Edit(Edit::Backspace));
			}
			CalcButton::Eval => {
				eval(&state.input.text(), &state.prec, &mut state.ouptut);
			}
		},
		Message::Edit(action) => state.input.perform(action),
		Message::EditPrec(prec) => state.prec = prec,
	}
}

fn view(state: &State) -> Element<Message> {
	column![
		row![
			text_editor(&state.input)
				.on_action(Message::Edit)
				.height(100),
			column![
				text_input("1024", &state.prec).width(60).on_input(Message::EditPrec),
				calc_button(CalcButton::Eval)
			],
		],
		text(&state.ouptut).height(100),
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
	prec: String,
	ouptut: String,
}

#[derive(Debug, Clone)]
enum Message {
	ButtonPressed(CalcButton),
	Edit(Action),
	EditPrec(String)
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

fn eval(input: &str, prec: &str, output: &mut String) {
	output.clear();
	let prec = prec.parse::<i64>().unwrap_or(1024);
	
	let tokens = Lexer(input).collect::<Vec<_>>();
	let mut parser = Parser::new(&tokens);
	let expr = match parser.parse() {
		Err(err) => {
			output.push_str(&err.to_string());
			return;
		},
		Ok(expr) => expr,
	};
	
	let res_str = std::panic::catch_unwind(|| {
		match expr.eval(prec) {
			Err(err) => err.to_string(),
			Ok(expr) => expr.to_string(prec),
		}
	});
	
	match res_str {
		Err(err) => output.push_str("Error"),
		Ok(res) => output.push_str(&res),
	}
}