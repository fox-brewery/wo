pub struct Defaults {
	pub win_x_width: f32,
	pub win_y_height: f32,
}

pub fn get_defaults() -> Defaults {
	Defaults {
		win_x_width: 720.0,
		win_y_height: 1080.0,
	}
}
