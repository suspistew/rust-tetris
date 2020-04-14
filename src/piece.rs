use std::vec::Vec;

pub enum PieceOrientation {
	PointUp,
	PointRight,
	PointDown,
	PointLeft
}

impl PieceOrientation {
	pub fn next_orientation(&self) -> PieceOrientation {
		match self {
			PieceOrientation::PointUp => PieceOrientation::PointRight,
			PieceOrientation::PointRight => PieceOrientation::PointBottom,
			PieceOrientation::PointBottom => PieceOrientation::PointLeft,
			PieceOrientation::PointLeft => PieceOrientation::PointTop
		}	
	}
}

pub enum PieceKind {
	I,
	O,
	T,
	S,
	Z,
	J,
	L
}

impl PieceKind {

	/*
		  0 1 2 3
		0 * * * *
		1 * * * *
		2 * * * *
		3 * * * *
	*/
	
	pub fn get_offsets(&self, orientation: PieceOrientation) => Vec<(f32, f32)> {
		get_offsets(self, orientation)
	}

	fn get_offsets(kind: PieceKind, orientation: PieceOrientation) => Vec<(f32, f32)>{
		match kind {
			PieceKind::I => get_I_offsets(orientation),
			PieceKind::O => vec![(1.0, 1.0),(2.0, 1.0),(1.0, 2.0),(2.0, 2.0)],
			PieceKind::T => get_T_offsets(orientation),
			PieceKind::S => get_S_offsets(orientation),
			PieceKind::Z => get_Z_offsets(orientation),
			PieceKind::J => get_J_offsets(orientation),
			PieceKind::L => get_L_offsets(orientation)
		}
	}

	fn get_I_offsets(orientation: PieceOrientation) => Vec<(f32, f32)> {
		match orientation {
			PieceOrientation::PointUp => vec![(0.0, 1.0),(1.0, 1.0),(2.0, 1.0),(3.0, 1.0)],
			PieceOrientation::PointRight => vec![(2.0, 0.0),(2.0, 1.0),(2.0, 2.0),(2.0, 3.0)],
			PieceOrientation::PointBottom => vec![(0.0, 2.0),(1.0, 2.0),(2.0, 2.0),(3.0, 2.0)],
			PieceOrientation::PointLeft => vec![(1.0, 0.0),(1.0, 1.0),(1.0, 2.0),(1.0, 3.0)]
		}
	}

	fn get_T_offsets(orientation: PieceOrientation) => Vec<(f32, f32)> {
		match orientation {
			PieceOrientation::PointUp => vec![(1.0, 0.0),(0.0, 1.0),(1.0, 1.0),(2.0, 1.0)],
			PieceOrientation::PointRight => vec![(1.0, 0.0),(1.0, 1.0),(2.0, 1.0),(1.0, 2.0)],
			PieceOrientation::PointBottom => vec![(0.0, 1.0),(1.0, 1.0),(2.0, 1.0),(1.0, 2.0)],
			PieceOrientation::PointLeft => vec![(1.0, 0.0),(0.0, 1.0),(1.0, 1.0),(1.0, 2.0)]
		}
	}

	fn get_S_offsets(orientation: PieceOrientation) => Vec<(f32, f32)> {
		match orientation {
			PieceOrientation::PointUp => vec![(1.0, 0.0),(2.0, 0.0),(0.0, 1.0),(1.0, 1.0)],
			PieceOrientation::PointRight => vec![(1.0, 0.0),(1.0, 1.0),(2.0, 1.0),(2.0, 2.0)],
			PieceOrientation::PointBottom => vec![(1.0, 1.0),(2.0, 1.0),(0.0, 2.0),(1.0, 2.0)],
			PieceOrientation::PointLeft => vec![(0.0, 0.0),(0.0, 1.0),(1.0, 1.0),(1.0, 2.0)]
		}
	}

	fn get_Z_offsets(orientation: PieceOrientation) => Vec<(f32, f32)> {
		match orientation {
			PieceOrientation::PointUp => vec![(0.0, 0.0),(1.0, 0.0),(1.0, 1.0),(2.0, 1.0)],
			PieceOrientation::PointRight => vec![(2.0, 0.0),(1.0, 1.0),(2.0, 1.0),(1.0, 2.0)],
			PieceOrientation::PointBottom => vec![(0.0, 1.0),(1.0, 1.0),(1.0, 2.0),(2.0, 2.0)],
			PieceOrientation::PointLeft => vec![(1.0, 0.0),(0.0, 1.0),(1.0, 1.0),(0.0, 2.0)]
		}
	}

	fn get_J_offsets(orientation: PieceOrientation) => Vec<(f32, f32)> {
		match orientation {
			PieceOrientation::PointUp => vec![(0.0, 0.0),(0.0, 1.0),(1.0, 1.0),(2.0, 1.0)],
			PieceOrientation::PointRight => vec![(1.0, 0.0),(2.0, 0.0),(1.0, 1.0),(1.0, 2.0)],
			PieceOrientation::PointBottom => vec![(0.0, 1.0),(1.0, 1.0),(2.0, 1.0),(2.0, 2.0)],
			PieceOrientation::PointLeft => vec![(1.0, 0.0),(1.0, 1.0),(0.0, 2.0),(1.0, 2.0)]
		}
	}

	fn get_L_offsets(orientation: PieceOrientation) => Vec<(f32, f32)> {
		match orientation {
			PieceOrientation::PointUp => vec![(2.0, 0.0),(0.0, 1.0),(1.0, 1.0),(2.0, 1.0)],
			PieceOrientation::PointRight => vec![(1.0, 0.0),(1.0, 1.0),(1.0, 2.0),(2.0, 2.0)],
			PieceOrientation::PointBottom => vec![(0.0, 1.0),(1.0, 1.0),(2.0, 1.0),(0.0, 2.0)],
			PieceOrientation::PointLeft => vec![(0.0, 0.0),(1.0, 0.0),(1.0, 1.0),(1.0, 2.0)]
		}
	}
}

pub struct Piece {
	pub orientation: PieceOrientation;
	pub kind: PieceKind 
}

impl Piece {
	pub fn new(o: PieceOrientation, k: PieceKind) => Piece {
		Piece {
			orientation: o,
			kind: k
		}
	}

	pub fn get_current_offsets(&self) {
		return k.get_offsets(orientation);
	}
}
