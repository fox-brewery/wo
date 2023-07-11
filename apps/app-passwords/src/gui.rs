use std::{collections::HashMap, iter::Map};

use anyhow::Result;
use eframe::egui::{self, RichText};
use walkdir::{DirEntry, FilterEntry, WalkDir};
use directories;

use wo_common as common;

pub fn run() -> Result<(), eframe::Error> {
	let defaults = common::get_defaults();

	let options = eframe::NativeOptions {
		initial_window_size: Some(egui::vec2(defaults.win_x_width, defaults.win_y_height)),
		..Default::default()
	};
	eframe::run_native(
		"app-passwords",
		options,
		Box::new(|_cc| Box::<AppPasswords>::default()),
	)
}

fn is_hidden(entry: &DirEntry) -> bool {
	entry
		.file_name()
		.to_str()
		.map(|s| s.contains(".git"))
		.unwrap_or(false)
}

struct AppPasswords {}

impl Default for AppPasswords {
	fn default() -> Self {
		Self {}
	}
}

impl eframe::App for AppPasswords {
	fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
		egui::CentralPanel::default().show(ctx, |ui| {
			let mut jobs_map: HashMap<String, Vec<String>> = HashMap::new();

			ui.heading(RichText::new("Password Management").size(24.0));
			ui.separator();
			ui.add_space(5.0);

			println!("RRR {:?}", jobs_map.len());
			for (k, v) in jobs_map.iter() {
				println!("{}: {:?}", k, v);
				ui.collapsing(k, |ui| {
					ui.heading(k);
					for i in v.iter() {
						ui.label(i);
					}
				});
			}

			if ui.button("Update").clicked() {
				let dir = directories::BaseDirs::new().unwrap().data_local_dir().join("password-store");

				jobs_map.clear();
				let walker = WalkDir::new(dir).into_iter();
				for entry in walker
					.filter_entry(|e| !is_hidden(e))
					.filter_map(|e| e.ok())
				{
					let path = entry.path().to_owned();
					let path = path.to_str().unwrap().to_owned();
					if path.contains("jobs/") {
						let k = entry.path().file_name().unwrap();
						let k = k.to_str().unwrap().to_owned();
						let k = k.trim_end_matches(".gpg").to_owned();

						println!("{:?}", jobs_map);
						jobs_map.insert(k.clone(), Vec::new());
					}
				}

				// println!("{} {}", "aaa", jobs_map.len());
				// println!("{:?}", jobs_map);
				// for (k, v) in jobs_map.iter() {
				// 	println!("{}: {:?}", k, v);
				// 	ui.collapsing(k, |ui| {
				// 		ui.heading(k);
				// 		for i in v.iter() {
				// 			ui.label(i);
				// 		}
				// 	});
				// }

				let walker = WalkDir::new(dir).into_iter();
				for entry in walker
					.filter_entry(|e| !is_hidden(e))
					.filter_map(|e| e.ok())
				{
					if entry.path().is_symlink() {
						println!("symlink: {}", entry.path().display());
					}
				}
			}
		});
	}
}
