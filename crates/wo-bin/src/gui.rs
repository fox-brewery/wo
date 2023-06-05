use std::process::{exit, Command};

use anyhow::Result;
use eframe::egui::{self, Button, Label, RichText, ScrollArea, TextEdit};

use wo_common as common;
use wo_defaults;

use crate::{repo, util};

pub struct RunGui;

enum CurrentView {
	DefaultView,
	OpenRepositoryView,
	KeybindingsView,
	ChooseAppsView,
}

pub fn run() -> Result<(), eframe::Error> {
	let defaults = wo_defaults::get_defaults();

	let options = eframe::NativeOptions {
		initial_window_size: Some(egui::vec2(defaults.win_x_width, defaults.win_y_height)),
		..Default::default()
	};
	eframe::run_native("repomgr", options, Box::new(|_cc| Box::<MyApp>::default()))
}

struct MyApp {
	current_view: CurrentView,
	search_txt: String,
	hovered_repo: Option<repo::Repo>,
}

impl Default for MyApp {
	fn default() -> Self {
		Self {
			current_view: CurrentView::DefaultView,
			search_txt: String::new(),
			hovered_repo: Option::None,
		}
	}
}

impl eframe::App for MyApp {
	fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
		egui::TopBottomPanel::top("top").show(ctx, |ui| {
			ui.horizontal(|ui| {
				ui.menu_button("File", |ui| {
					if ui.button("Quit").clicked() {
						exit(1);
					}
				});

				ui.menu_button("Edit", |ui| {
					if ui.button("Copy Binary").clicked() {
						util::write_desktop_entry().expect("Failed to write desktop entry");
					}
				});
			})
		});

		egui::CentralPanel::default().show(ctx, |ui| {
			ui.heading(RichText::new("wo").size(24.0));
			ui.separator();
			ui.add_space(5.0);

			// ScrollArea::vertical().show(ui, |ui| {
			ui.columns(3, |cols| {
				cols[0].group(|ui| {
					if ui.button("Open Repository").clicked() {
						self.current_view = CurrentView::OpenRepositoryView;
						let bin = common::get_bin_file("app-repositories");

						common::fork_gui_foreground(&bin.to_string_lossy(), &[]);
					}
				});
				cols[1].group(|ui| {
					if ui.button("Configure Default Apps (choose)").clicked() {
						self.current_view = CurrentView::ChooseAppsView;
					}
				});
				cols[1].group(|ui| {
					if ui.button("Update Keybindings").clicked() {
						self.current_view = CurrentView::KeybindingsView;
					}
				});
			});
			// });
			ui.separator();

			match self.current_view {
				CurrentView::DefaultView => {
					ui.heading("This is the default view");
				}
				CurrentView::OpenRepositoryView => {
					let repos = repo::get_repos().unwrap();

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
								ui.label(repo.name);
								ui.label(repo.path.to_string_lossy());
								if ui.button("Open").clicked() {
									Command::new("code").arg(repo.path.clone()).spawn().unwrap();
									println!("Opening {}", repo.path.display())
								}
							}
						});
					});
				}
				CurrentView::KeybindingsView => {
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
				}
				CurrentView::ChooseAppsView => {}
			}
		});
	}
}
