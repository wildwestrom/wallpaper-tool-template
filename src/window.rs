use std::rc::Rc;

use lib::is_chinese_character;
use relm4::{adw, adw::prelude::*, component, gtk, prelude::*, SimpleComponent};

const TITLE: &str = "漢tracker";

use super::input as input_screen;
use super::result as result_screen;
use super::testing as testing_screen;

#[derive(Debug)]
pub struct Ht {
	view_stack: Rc<adw::ViewStack>,
	text_to_test: String,
	input_screen: Controller<input_screen::InputScreen>,
	testing_screen: Controller<testing_screen::TestingScreen>,
	result_screen: Controller<result_screen::ResultScreen>,
	db: Rc,
}

#[component(pub)]
impl SimpleComponent for Ht {
	type Init = ();
	type Input = NextScreen;
	type Output = ();

	view! {
		#[root]
		adw::Window::builder()
			.title(TITLE)
			.default_width(320)
			.default_height(240)
			.mnemonics_visible(false)
			.deletable(true)
			.resizable(true)
			.build() {
			#[wrap(Some)]
			set_content = &adw::Clamp {
				set_css_classes: &["m-8"],
				set_overflow: gtk::Overflow::Visible,
				set_orientation: gtk::Orientation::Horizontal,
				set_valign: gtk::Align::Fill,
				set_halign: gtk::Align::Fill,
				set_unit: adw::LengthUnit::Px,
				set_maximum_size: 1920,
				set_tightening_threshold: 1000,

				#[local_ref]
				view_stack -> adw::ViewStack {
					set_hhomogeneous: false,
					add = model.input_screen.widget(),
					add = model.testing_screen.widget(),
					add = model.result_screen.widget(),
				},
			}
		}
	}

	fn init(
		_init: Self::Init,
		widgets: Self::Root,
		sender: ComponentSender<Self>,
	) -> ComponentParts<Self> {
		let view_stack = Rc::new(adw::ViewStack::new());

		let input_screen =
			input_screen::InputScreen::builder()
				.launch(())
				.forward(sender.input_sender(), |msg| match msg {
					input_screen::OutputMessage::ResumeTest => {
						// todo
						NextScreen::Testing("string from sql DB".to_string())
					}
					input_screen::OutputMessage::NewTest(s) => NextScreen::Testing(s),
				});

		let testing_screen = testing_screen::TestingScreen::builder().launch(()).forward(
			sender.input_sender(),
			|msg| match msg {
				testing_screen::OutputMessage::Finish(chars) => NextScreen::Results(chars),
			},
		);

		let result_screen = result_screen::ResultScreen::builder().launch(()).forward(
			sender.input_sender(),
			|msg| match msg {
				result_screen::OutputMessage::StartOver => NextScreen::Input,
				result_screen::OutputMessage::Exit => NextScreen::Exit,
			},
		);

		let model = Self {
			view_stack,
			input_screen,
			text_to_test: String::new(),
			testing_screen,
			result_screen,
		};

		let view_stack = &*model.view_stack;

		let widgets = view_output!();

		ComponentParts { model, widgets }
	}

	fn update(&mut self, message: Self::Input, _sender: ComponentSender<Self>) {
		match message {
			NextScreen::Input => {
				let input_screen_widget = self.input_screen.widget();
				self.view_stack.set_visible_child(input_screen_widget);
			}
			NextScreen::Testing(text) => {
				self.text_to_test = text;
				let testing_screen_widget = self.testing_screen.widget();
				let chars: Vec<_> = self
					.text_to_test
					.chars()
					.filter(is_chinese_character)
					.collect();
				self.testing_screen
					.sender()
					.send(testing_screen::Message::StartTest(chars))
					.expect("Shouldn't fail");
				self.view_stack.set_visible_child(testing_screen_widget);
			}
			NextScreen::Results(chars) => {
				let result_screen_widget = self.result_screen.widget();
				self.result_screen
					.sender()
					.send(result_screen::Message::ShowResults(
						self.text_to_test.clone(),
						chars,
					))
					.expect("Shouldn't fail");
				self.view_stack.set_visible_child(result_screen_widget);
			}
			NextScreen::Exit => relm4::main_adw_application().quit(),
		}
	}
}

#[derive(Debug, Clone)]
pub enum NextScreen {
	Input,
	Testing(String),
	Results(Vec<char>),
	Exit,
}
