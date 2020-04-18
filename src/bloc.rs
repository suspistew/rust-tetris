use amethyst::ecs::prelude::{Component, DenseVecStorage};

#[derive(Debug)]
pub enum BlocKind {
    Border,
    Moving,
    Static,
}

#[derive(Debug)]
pub struct Bloc {
    pub kind: BlocKind,
}

impl Bloc {
    pub fn new(k: BlocKind) -> Bloc {
        Bloc { kind: k }
    }
}

impl Component for Bloc {
    type Storage = DenseVecStorage<Self>;
}
