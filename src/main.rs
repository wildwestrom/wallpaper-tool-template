use relm4::adw::prelude::*;
use relm4::{adw, gtk, prelude::*};

mod input;
mod result;
mod testing;
mod window;

const APP_ID: &str = "xyz.westrom.hantracker";
fn load_css() {
	let css_provider = gtk::CssProvider::new();
	css_provider.load_from_path("ui/main.css");

	gtk::style_context_add_provider_for_display(
		&gtk::gdk::Display::default().expect("Could not connect to display"),
		&css_provider,
		gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
	);
}

fn main() {
	let adw_app = adw::Application::builder().application_id(APP_ID).build();
	adw_app.connect_startup(|_| load_css());
	let app = RelmApp::from_app(adw_app);
	app.run::<window::Ht>(());

	// // Use for cleaning up input data.
	// let chars = include_str!("common_cn_tier_1.txt");
	// let mut chars: Vec<char> = chars.chars().filter(lib::is_chinese_character).collect();
	// chars.dedup();
	// print!("{}", chars.into_iter().collect::<String>());
}
