use std::fs;

use anyhow::{Ok, Result};

pub fn write_desktop_entry() -> Result<()> {
	let desktop_file = directories::BaseDirs::new()
		.expect("Failed to get BaseDirs")
		.data_dir()
		.join("applications")
		.join("wo.desktop");

	// TODO: desktop-file-install --reubild-mime-info-cache
	let content = "[Desktop Entry]
Type=Application
Version=1.5
Name=wo
Comment=A wonderful organizer
Exec=/storage/ur/storage_home/Docs/Programming/Repositories/default/wo/target/debug/wo-bin
Icon=/storage/ur/storage_home/Docs/Programming/Repositories/default/wo/share/wo.png
Terminal=false
Categories=Development
Keywords=woo,root,app,wwoo,wowo,wow

";

	fs::write(desktop_file, content)?;

	Ok(())
}
