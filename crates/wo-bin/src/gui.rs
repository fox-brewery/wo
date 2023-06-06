use std::process::exit;

use anyhow::Result;
use eframe::{
	egui::{self, RichText},
	epaint::Vec2,
};

use wo_common as common;
use wo_defaults;

use crate::util;

pub fn run() -> Result<(), eframe::Error> {
	let defaults = wo_defaults::get_defaults();

	let options = eframe::NativeOptions {
		initial_window_size: Some(egui::vec2(defaults.win_x_width, defaults.win_y_height)),
		..Default::default()
	};
	eframe::run_native("wo", options, Box::new(|_cc| Box::<MyApp>::default()))
}

struct MyApp {}

impl Default for MyApp {
	fn default() -> Self {
		Self {}
	}
}

impl eframe::App for MyApp {
	fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
		let make_grid_item = |ui: &mut egui::Ui, button_text: &str| {
			egui::Frame::default()
				.rounding(5.0)
				.fill(egui::Color32::from_rgb(33, 37, 41))
				.show(ui, |ui| {
					ui.set_width(150.0);
					ui.set_height(150.0);

					ui.centered_and_justified(|ui| {
						if ui
							.button(RichText::new(button_text).size(16.0).strong())
							.clicked()
						{
							let bin = common::get_bin_file(button_text);

							common::fork_gui_foreground(&bin.to_string_lossy(), [].to_vec());
						}
					});
				});
		};

		egui::TopBottomPanel::top("top").show(ctx, |ui| {
			ui.horizontal(|ui| {
				ui.menu_button("File", |ui| {
					if ui.button("Quit").clicked() {
						exit(1);
					}
				});

				ui.menu_button("Edit", |ui| {
					if ui.button("Copy Binary and Desktop Entry").clicked() {
						util::write_desktop_entry().expect("Failed to write desktop entry");
					}
				});
			})
		});

		egui::CentralPanel::default().show(ctx, |ui| {
			ui.heading(RichText::new("wo").size(24.0));
			ui.separator();
			ui.add_space(5.0);

			egui::Grid::new("app-grid")
				.spacing(Vec2::new(10.0, 10.0))
				.show(ui, |ui| {
					make_grid_item(ui, "app-repositories");
					make_grid_item(ui, "app-keybindings");
					make_grid_item(ui, "app-project");
					ui.end_row();

					make_grid_item(ui, "context-shell");
					make_grid_item(ui, "context-project");
					// make_grid_item(ui, "app-");
					ui.end_row();
				});
		});
	}
}
