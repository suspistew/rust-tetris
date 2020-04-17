use std::vec::Vec;
use rand::{thread_rng, Rng};

#[derive(Debug)]
pub enum PieceOrientation {
    PointUp,
    PointRight,
    PointDown,
    PointLeft,
}

impl PieceOrientation {
    pub fn next_orientation(&self) -> PieceOrientation {
        match self {
            PieceOrientation::PointUp => PieceOrientation::PointRight,
            PieceOrientation::PointRight => PieceOrientation::PointDown,
            PieceOrientation::PointDown => PieceOrientation::PointLeft,
            PieceOrientation::PointLeft => PieceOrientation::PointUp,
        }
    }
}

#[repr(u8)]
#[derive(Debug)]
pub enum PieceKind {
    I,
    O,
    T,
    S,
    Z,
    J, 
    L,
}

impl PieceKind {

    pub fn from_int(x: u8) -> Result<PieceKind, &'static str>{
        match x {
            x if x == PieceKind::I as u8 => Ok(PieceKind::I),
            x if x == PieceKind::O as u8 => Ok(PieceKind::O),
            x if x == PieceKind::T as u8 => Ok(PieceKind::T),
            x if x == PieceKind::S as u8 => Ok(PieceKind::S),
            x if x == PieceKind::Z as u8 => Ok(PieceKind::Z),
            x if x == PieceKind::J as u8 => Ok(PieceKind::J),
            x if x == PieceKind::L as u8 => Ok(PieceKind::L),
            _ => Err("Error while convertine u8 to PieceKind"),
        }
    }
    /*
          0 1 2 3
        0 * * * *
        1 * * * *
        2 * * * *
        3 * * * *
    */

    pub fn get_self_offsets(&self, orientation: &PieceOrientation) -> Vec<(f32, f32)> {
        PieceKind::get_offsets(self, &orientation)
    }

    fn get_offsets(kind: &PieceKind, orientation: &PieceOrientation) -> Vec<(f32, f32)> {
        match kind {
            PieceKind::I => PieceKind::get_i_offsets(orientation),
            PieceKind::O => vec![(1.0, 1.0), (2.0, 1.0), (1.0, 2.0), (2.0, 2.0)],
            PieceKind::T => PieceKind::get_t_offsets(orientation),
            PieceKind::S => PieceKind::get_s_offsets(orientation),
            PieceKind::Z => PieceKind::get_z_offsets(orientation),
            PieceKind::J => PieceKind::get_j_offsets(orientation),
            PieceKind::L => PieceKind::get_l_offsets(orientation),
        }
    }

    fn get_i_offsets(orientation: &PieceOrientation) -> Vec<(f32, f32)> {
        match orientation {
            PieceOrientation::PointUp => vec![(0.0, 1.0), (1.0, 1.0), (2.0, 1.0), (3.0, 1.0)],
            PieceOrientation::PointRight => vec![(2.0, 0.0), (2.0, 1.0), (2.0, 2.0), (2.0, 3.0)],
            PieceOrientation::PointDown => vec![(0.0, 2.0), (1.0, 2.0), (2.0, 2.0), (3.0, 2.0)],
            PieceOrientation::PointLeft => vec![(1.0, 0.0), (1.0, 1.0), (1.0, 2.0), (1.0, 3.0)],
        }
    }

    fn get_t_offsets(orientation: &PieceOrientation) -> Vec<(f32, f32)> {
        match orientation {
            PieceOrientation::PointUp => vec![(1.0, 0.0), (0.0, 1.0), (1.0, 1.0), (2.0, 1.0)],
            PieceOrientation::PointRight => vec![(1.0, 0.0), (1.0, 1.0), (2.0, 1.0), (1.0, 2.0)],
            PieceOrientation::PointDown => vec![(0.0, 1.0), (1.0, 1.0), (2.0, 1.0), (1.0, 2.0)],
            PieceOrientation::PointLeft => vec![(1.0, 0.0), (0.0, 1.0), (1.0, 1.0), (1.0, 2.0)],
        }
    }

    fn get_s_offsets(orientation: &PieceOrientation) -> Vec<(f32, f32)> {
        match orientation {
            PieceOrientation::PointUp => vec![(1.0, 0.0), (2.0, 0.0), (0.0, 1.0), (1.0, 1.0)],
            PieceOrientation::PointRight => vec![(1.0, 0.0), (1.0, 1.0), (2.0, 1.0), (2.0, 2.0)],
            PieceOrientation::PointDown => vec![(1.0, 1.0), (2.0, 1.0), (0.0, 2.0), (1.0, 2.0)],
            PieceOrientation::PointLeft => vec![(0.0, 0.0), (0.0, 1.0), (1.0, 1.0), (1.0, 2.0)],
        }
    }

    fn get_z_offsets(orientation: &PieceOrientation) -> Vec<(f32, f32)> {
        match orientation {
            PieceOrientation::PointUp => vec![(0.0, 0.0), (1.0, 0.0), (1.0, 1.0), (2.0, 1.0)],
            PieceOrientation::PointRight => vec![(2.0, 0.0), (1.0, 1.0), (2.0, 1.0), (1.0, 2.0)],
            PieceOrientation::PointDown => vec![(0.0, 1.0), (1.0, 1.0), (1.0, 2.0), (2.0, 2.0)],
            PieceOrientation::PointLeft => vec![(1.0, 0.0), (0.0, 1.0), (1.0, 1.0), (0.0, 2.0)],
        }
    }

    fn get_j_offsets(orientation: &PieceOrientation) -> Vec<(f32, f32)> {
        match orientation {
            PieceOrientation::PointUp => vec![(0.0, 0.0), (0.0, 1.0), (1.0, 1.0), (2.0, 1.0)],
            PieceOrientation::PointRight => vec![(1.0, 0.0), (2.0, 0.0), (1.0, 1.0), (1.0, 2.0)],
            PieceOrientation::PointDown => vec![(0.0, 1.0), (1.0, 1.0), (2.0, 1.0), (2.0, 2.0)],
            PieceOrientation::PointLeft => vec![(1.0, 0.0), (1.0, 1.0), (0.0, 2.0), (1.0, 2.0)],
        }
    }

    fn get_l_offsets(orientation: &PieceOrientation) -> Vec<(f32, f32)> {
        match orientation {
            PieceOrientation::PointUp => vec![(2.0, 0.0), (0.0, 1.0), (1.0, 1.0), (2.0, 1.0)],
            PieceOrientation::PointRight => vec![(1.0, 0.0), (1.0, 1.0), (1.0, 2.0), (2.0, 2.0)],
            PieceOrientation::PointDown => vec![(0.0, 1.0), (1.0, 1.0), (2.0, 1.0), (0.0, 2.0)],
            PieceOrientation::PointLeft => vec![(0.0, 0.0), (1.0, 0.0), (1.0, 1.0), (1.0, 2.0)],
        }
    }
}

#[derive(Debug)]
pub struct Piece {
    pub orientation:  PieceOrientation,
    pub kind: PieceKind,
}

impl Piece {
    pub fn new(o: PieceOrientation, k: PieceKind) -> Piece {
        Piece {
            orientation: o,
            kind: k,
        }
    }

    pub fn get_current_offsets(&self) -> Vec<(f32,f32)>{
        self.kind.get_self_offsets(&self.orientation)
    }

    pub fn random_new() -> Piece{
        let random_piece_nb:u8 = thread_rng().gen_range(0,7);
        let piece_kind = PieceKind::from_int(random_piece_nb).unwrap();
        Piece {
            orientation:  PieceOrientation::PointUp,
            kind: piece_kind
        }
    }
}
