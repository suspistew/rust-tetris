use amethyst::core::{
	timing::Time,
	SystemDesc, Transform};
use amethyst::derive::SystemDesc;
use amethyst::ecs::{Join, Read, ReadStorage, System, SystemData, World, WriteStorage};
use amethyst::input::{InputHandler, StringBindings};

use crate::bloc::Bloc;

#[derive(SystemDesc)]
pub struct BlocSystem;

impl<'s> System<'s> for BlocSystem {
    type SystemData = (ReadStorage<'s, Bloc>);

    fn run(&mut self, (blocs): Self::SystemData) {
	}
}
