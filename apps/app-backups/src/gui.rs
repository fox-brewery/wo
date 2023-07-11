use std::sync::mpsc::channel;
use std::sync::mpsc::{self, Receiver, Sender};
use std::thread;
use std::time::Duration;
use std::{
	collections::HashMap,
	io::{BufRead, BufReader},
	iter::Map,
	process::{Child, Command, Stdio},
};

use anyhow::Result;
use eframe::egui::{self, RichText, ScrollArea};
use walkdir::{DirEntry, FilterEntry, WalkDir};

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

struct AppPasswords {
	output1: String,
	output1cmd: Option<Child>,
	tx: Sender<String>,
	rx: Receiver<String>,
}

impl Default for AppPasswords {
	fn default() -> Self {
		let (tx, rx) = mpsc::channel::<String>();

		Self {
			output1: "first".to_string(),
			output1cmd: Option::None,
			tx,
			rx,
		}
	}
}

impl eframe::App for AppPasswords {
	fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
		egui::CentralPanel::default().show(ctx, |ui| {
			ui.heading(RichText::new("Backup Management").size(24.0));
			ui.separator();
			ui.add_space(5.0);

			ui.heading("/storage/other");
			if ui.button("Backup Now").clicked() {
				if !self.output1cmd.is_some() {
					thread::spawn(|| {
						let val = String::from("hi");
						self.tx.send(val).unwrap();
					});

					self.output1cmd = Option::Some(
						Command::new("bash")
							.stdout(Stdio::piped())
							.stderr(Stdio::piped())
							.arg("-c")
							.arg("for n in {1..100}; do printf '%s\\n' \"$n\"; done")
							// .arg("while :; do echo '$RANDOM'; sleep 1; done")
							.spawn()
							.expect("Failed to execute process"),
					);
				}
			}
			if let Some(child) = &mut self.output1cmd {
				// child.wait_with_output().unwrap();

				println!("some");
				let stdout = child.stdout.as_mut().unwrap();
				let stdout_reader = BufReader::new(stdout);
				let stdout_lines = stdout_reader.buffer().lines();

				for line in stdout_lines {
					println!("Read: {:?}", line);
					self.output1 += line.unwrap().as_str().clone();
				}

				{
					let stdout = child.stderr.as_mut().unwrap();
					let stdout_reader = BufReader::new(stdout);
					let stdout_lines = stdout_reader.buffer().lines();

					for line in stdout_lines {
						println!("Read: {:?}", line);
						self.output1 += line.unwrap().as_str().clone();
					}
				}
			}
			ScrollArea::vertical()
				.max_height(300.0)
				.stick_to_bottom(true)
				.show(ui, |ui| {
					ui.set_width(ui.available_width());
					ui.text_edit_multiline(&mut self.output1);
				});

			ui.heading("/home/edwin");
			if ui.button("Backup Now").clicked() {}
		});
	}
}
