use amethyst::derive::SystemDesc;
use amethyst::ecs::{ReadStorage, System, SystemData};

use crate::bloc::Bloc;

#[derive(SystemDesc)]
pub struct BlocSystem;

impl<'s> System<'s> for BlocSystem {
    type SystemData = ReadStorage<'s, Bloc>;

    fn run(&mut self, _blocs: Self::SystemData) {}
}
