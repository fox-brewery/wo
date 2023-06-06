use std::process::Command;

use anyhow::Result;
use eframe::egui::{self, RichText, ScrollArea};

use wo_common as common;
use wo_defaults;

pub fn run() -> Result<(), eframe::Error> {
	let defaults = wo_defaults::get_defaults();

	let options = eframe::NativeOptions {
		initial_window_size: Some(egui::vec2(defaults.win_x_width, defaults.win_y_height)),
		..Default::default()
	};
	eframe::run_native(
		"app-repositories",
		options,
		Box::new(|_cc| Box::<AppRepositories>::default()),
	)
}

struct AppRepositories {
	search_txt: String,
	hovered_repo: Option<common::Repo>,
}

impl Default for AppRepositories {
	fn default() -> Self {
		Self {
			search_txt: String::new(),
			hovered_repo: Option::None,
		}
	}
}

impl eframe::App for AppRepositories {
	fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
		egui::CentralPanel::default().show(ctx, |ui| {
			ui.heading(RichText::new("App Repository").size(24.0));
			ui.separator();
			ui.add_space(5.0);

			let repos = common::get_repos().unwrap();

			ui.columns(2, |cols| {
				cols[0].group(|ui| {
					ui.text_edit_singleline(&mut self.search_txt).changed();
					ScrollArea::vertical().show(ui, |ui| {
						for repo in repos {
							if self.search_txt.is_empty() || repo.name.contains(&self.search_txt) {
								ui.horizontal(|ui| {
									if ui.label(&repo.name).hovered() {
										self.hovered_repo = Option::Some(repo.clone());
									}
								});
							}
						}
					});
				});
				cols[1].group(|ui| {
					if let Some(repo) = self.hovered_repo.clone() {
						ui.heading("Info");
						ui.label(repo.name);
						ui.label(repo.path.to_string_lossy());
						if ui.button("Open").clicked() {
							Command::new("code").arg(repo.path.clone()).spawn().unwrap();
							println!("Opening {}", repo.path.display())
						}
					}
				});
			});
		});
	}
}
