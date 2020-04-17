use amethyst::derive::SystemDesc;
use amethyst::ecs::{ReadStorage, System, SystemData, prelude::Entities};

use crate::piece::{Piece, PieceKind, PieceOrientation};
use crate::bloc::{Bloc, FallingBloc};

#[derive(SystemDesc)]
pub struct PieceSystem {
    #[system_desc(skip)]
    active_piece: Piece,
    #[system_desc(skip)]
    next_piece: Piece,
}

impl PieceSystem {
    pub fn new() -> PieceSystem {
        PieceSystem {
            active_piece: Piece::random_new(),
            next_piece: Piece::random_new(),
        }
    }
}

impl<'s> System<'s> for PieceSystem {
    type SystemData = (
        ReadStorage<'s, FallingBloc>,
        ReadStorage<'s, Bloc>,
        Entities<'s>,
    );        
    
    fn run(&mut self, (_falling_blocs, _allblocs, _entities): Self::SystemData) {
        println!("generated active {:?}", self.active_piece);
        println!("generated next {:?}", self.next_piece);
    }
}
