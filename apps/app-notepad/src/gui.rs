use std::{process::Command, path::PathBuf, fs::{self, File, OpenOptions}};

use anyhow::Result;
use eframe::egui;
use directories;

use wo_common as common;

pub fn run() -> Result<(), eframe::Error> {
	let defaults = common::get_defaults();

	let options = eframe::NativeOptions {
		initial_window_size: Some(egui::vec2(defaults.win_x_width, defaults.win_y_height)),
		..Default::default()
	};
	eframe::run_native(
		"app-notepad",
		options,
		Box::new(|_cc| Box::<AppRepositories>::default()),
	)
}

struct AppRepositories {
	file: PathBuf,
	content: String,
}

impl Default for AppRepositories {
	fn default() -> Self {
		let file = directories::BaseDirs::new().unwrap().data_dir().join("wo/app-notepad-content.txt");

		fs::create_dir_all(&file.parent().unwrap()).unwrap();
		File::create_new(&file);

		Self {
			file: file.clone(),
			content: fs::read_to_string(&file).unwrap_or_default(),
		}
	}
}

impl eframe::App for AppRepositories {
	fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
		egui::CentralPanel::default().show(ctx, |ui| {
			ui.heading("app-notepad");

			// TODO: Not working
			ui.set_width(ui.available_width());
			ui.set_height(ui.available_height());

			if ui.text_edit_multiline(&mut self.content).changed() {
				fs::write(&self.file, &self.content).unwrap();
			}
		});
	}
}
