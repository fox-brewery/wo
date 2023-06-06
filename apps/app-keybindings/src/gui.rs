use std::process::Command;

use anyhow::Result;
use eframe::egui;

use wo_common as common;

pub fn run() -> Result<(), eframe::Error> {
	let defaults = common::get_defaults();

	let options = eframe::NativeOptions {
		initial_window_size: Some(egui::vec2(defaults.win_x_width, defaults.win_y_height)),
		..Default::default()
	};
	eframe::run_native(
		"app-keybindings",
		options,
		Box::new(|_cc| Box::<AppRepositories>::default()),
	)
}

struct AppRepositories {}

impl Default for AppRepositories {
	fn default() -> Self {
		Self {}
	}
}

impl eframe::App for AppRepositories {
	fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
		egui::CentralPanel::default().show(ctx, |ui| {
			ui.heading("This is the keybindings view");

			if ui.button("Transform").clicked() {
				Command::new("dconf")
					.arg("write")
					.arg("/org/mate/marco/global-keybindings/switch-to-workspace-left")
					.arg("'<Primary><Mod4>Left'")
					.spawn()
					.unwrap();
				Command::new("dconf")
					.arg("write")
					.arg("/org/mate/marco/global-keybindings/switch-to-workspace-right")
					.arg("'<Primary><Mod4>Right'")
					.spawn()
					.unwrap();
			}
		});
	}
}
