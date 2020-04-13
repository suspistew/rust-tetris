use std::vec::Vec;
use crate::bloc::{Bloc, Color}

pub const BOARD_HEIGHT: u32 = 12;
pub const BOARD_WIDTH: u32 = 22;

pub struct Board {
	pub blocs: Vec<Bloc>,	
}

impl Board {
	pub fn new() -> Board {
		Board {
			blocs: Vec::new()
		}	
	}
}
