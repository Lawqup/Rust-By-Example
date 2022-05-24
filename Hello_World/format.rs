use std::fmt::{self, Display, Formatter};

struct Color {
	red: u8,
	green: u8,
	blue: u8
}

impl Display for Color {
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		write!(f, "RGB ({r}, {g}, {b}) 0x{r:02x}{g:02x}{b:02x}",
			r = &self.red,
			g = &self.green,
			b = &self.blue)
	}
}

fn main() {

	let colors = [Color{ red: 255, green: 255, blue: 60},
				  Color{ red: 210, green: 120, blue: 0},
				  Color{red: 0, green: 0, blue: 0}];
	
	for color in colors.iter() {
		println!("{}", *color);
	}
}
