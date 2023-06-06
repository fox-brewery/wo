mod gui;
mod util;

fn main() {
	let has_forked = std::env::var_os("WO_FORKED");
	if has_forked.is_some() {
		gui::run().unwrap();
	} else {
		let cmd = std::env::current_exe().expect("Failed to get current exe");
		let args = std::env::args_os()
			.into_iter()
			.map(|item| item.to_string_lossy().into_owned().to_string())
			.collect();

		std::env::set_var("WO_FORKED", "");

		wo_common::fork_gui_foreground(&cmd.to_string_lossy(), args);
	}
}
