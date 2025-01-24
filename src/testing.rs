use relm4::adw::prelude::*;
use relm4::component;
use relm4::gtk;
use relm4::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Test {
	char: char,
	recalled: Recalled,
}

#[derive(Debug)]
pub struct TestingScreen {
	chars: Vec<Test>,
	current_char: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Recalled {
	Unknown,
	Known,
}

#[component(pub)]
impl SimpleComponent for TestingScreen {
	type Init = ();
	type Input = Message;
	type Output = OutputMessage;

	view! {
		#[root]
		gtk::Box {
			set_orientation: gtk::Orientation::Vertical,
			set_hexpand: true,
			gtk::Label {
				set_css_classes: &["heading"],
				set_label: "Test your knowledge",
			},
			gtk::Label {
				set_css_classes: &["mb-2"],
				#[watch]
				set_label: &format!("Do you know the meaning of this character?\n{} of {}",
					model.current_char + 1,
					model.chars.len()
				),
			},
			gtk::Box {
				set_homogeneous: true,
				// set_column_spacing: 16,
				gtk::Revealer {
					#[watch]
					set_reveal_child: model.current_char > 0,
					set_transition_type: gtk::RevealerTransitionType::SwingLeft,
						gtk::Frame {
							set_margin_horizontal: 16,
							set_overflow: gtk::Overflow::Visible,
							gtk::Box {
							set_css_classes: &["p-8"],
							set_orientation: gtk::Orientation::Vertical,
							gtk::Label {
								set_css_classes: &["heading"],
								set_label: "Previous Character",
							},
							gtk::Label {
								set_css_classes: &["text-5xl"],
								#[watch]
								set_label: &model.chars.get(model.current_char.saturating_sub(1)).map_or_else(|| "No previous character.".into(), |q| q.char.to_string())
							},
							gtk::Button {
								set_css_classes: &[],
								set_label: "Go back",
								connect_clicked => {
									Message::GoBack
								}
							}
						}
					}
				},
				gtk::Label {
					set_css_classes: &["card", "text-9xl", "p-8"],
					#[watch]
					set_label: &model.chars.get(model.current_char).map_or_else(|| "No previous character.".into(), |q| q.char.to_string())
				},
				gtk::Separator {
					set_css_classes: &["spacer"],
				},
			},
			gtk::Box {
				set_css_classes: &["m-8", "linked"],
				set_orientation: gtk::Orientation::Horizontal,
				set_hexpand: true,
				set_halign: gtk::Align::Fill,
				set_homogeneous: true,
				gtk::Button {
					set_css_classes: &["error"],
					set_label: "I don't know",
					connect_clicked => Message::Answer(Recalled::Unknown),
				},
				gtk::Button {
					set_css_classes: &["success"],
					set_label: "I know it",
					connect_clicked => Message::Answer(Recalled::Known),
				}
			}
		}
	}

	fn init(
		_init: Self::Init,
		widgets: Self::Root,
		_sender: ComponentSender<Self>,
	) -> ComponentParts<Self> {
		let model = Self {
			chars: Vec::new(),
			current_char: 0,
		};

		let widgets = view_output!();

		ComponentParts { model, widgets }
	}

	fn update(&mut self, message: Self::Input, sender: ComponentSender<Self>) {
		match message {
			Message::StartTest(chars) => {
				use rand::seq::SliceRandom;
				let mut rng = rand::thread_rng();
				self.chars = chars
					.into_iter()
					.map(|c| Test {
						char: c,
						recalled: Recalled::Unknown,
					})
					.collect::<Vec<_>>();
				self.chars.dedup();
				self.chars.shuffle(&mut rng);
				self.current_char = 0;
			}
			Message::Finish(known_chars) => sender
				.output(OutputMessage::Finish(known_chars))
				.expect("sending finished failed"),
			Message::GoBack => {
				self.current_char -= 1;
			}
			Message::Answer(recalled) => {
				let finish = |chars: &[Test]| {
					sender.input(Message::Finish(
						chars
							.iter()
							.filter_map(|c| match c.recalled {
								Recalled::Known => Some(c.char),
								Recalled::Unknown => None,
							})
							.collect(),
					));
				};
				if let Some(c) = self.chars.get_mut(self.current_char) {
					c.recalled = recalled;
					self.current_char += 1;
					if self.current_char == self.chars.len() {
						finish(&self.chars);
					}
				} else {
					finish(&self.chars);
				};
			}
		}
	}
}

#[derive(Debug, Clone)]
pub enum Message {
	StartTest(Vec<char>),
	Finish(Vec<char>),
	Answer(Recalled),
	GoBack,
}

#[derive(Debug, Clone)]
pub enum OutputMessage {
	Finish(Vec<char>),
}
