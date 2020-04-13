use amethyst::ecs::prelude::{Component, DenseVecStorage};

pub enum Color {
	Gray,
	White,
	Black,
	Red,
	Yellow,
	Brown,
	Orange,
	Green,
	Cyan,
	Blue,
	Purple,
	Wtf
}

pub struct Bloc {
	pub color: Color,
}

impl Bloc {
	pub fn new(c: Color) -> Bloc {
		Bloc  {
			color: c,
		}
	}
}

impl Component for Bloc {
    type Storage = DenseVecStorage<Self>;
}
