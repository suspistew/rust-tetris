use crate::bloc::{Bloc, BlocKind, BLOC_SIZE};
use crate::piece::{PieceKind, PieceSystemState};
use crate::tetris::{TetrisResource};
use amethyst::core::{Transform};
use amethyst::derive::SystemDesc;
use amethyst::ecs::{Join, Read, ReadStorage, Entities, System, SystemData, Write, WriteStorage};
use amethyst::input::{InputHandler, StringBindings};
use amethyst::renderer::{SpriteRender, SpriteSheet};
use amethyst::assets::Handle;
use std::collections::HashMap;
use amethyst::core::timing::Time;



#[derive(SystemDesc)]
pub struct ScoringSystem;

impl<'s> System<'s> for ScoringSystem {
    type SystemData = (
        WriteStorage<'s, Transform>,
        ReadStorage<'s, Bloc>,
        Write<'s, TetrisResource>,
        Entities<'s>,
    );

    fn run(&mut self, (mut transforms, blocs, mut tetris_resource, mut entities): Self::SystemData) {
        let mut lines = HashMap::new();
        for (bloc, transform) in (&blocs, &mut transforms).join() {
            match bloc.kind {
                BlocKind::Static => {
                    let key = (transform.translation().y / BLOC_SIZE) as usize;
                    let new_val = match lines.get(&key) {
                        Some(val) => val + 1,
                        None => 1
                    };
                    lines.insert(key, new_val);
                },
                _ => {}
            }
        }
        let lines = {
            let mut full_lines = Vec::new();
            for (key, val) in lines.iter() {
                if val == &10 {
                    full_lines.push(*key);
                }
            }
            full_lines.sort();
            full_lines
        };

        if lines.len() > 0 {
            for (bloc, transform, entity) in (&blocs, &mut transforms, &*entities).join() {
                match bloc.kind {
                    BlocKind::Static => {
                        let line = (transform.translation().y / BLOC_SIZE) as usize;
                        if lines.contains(&line) {
                            entities.delete(entity);
                        }
                    },
                    _ => {}
                };
            } 
            for (index, line) in lines.iter().enumerate() {
                for (bloc, transform) in (&blocs, &mut transforms).join() {
                    match bloc.kind {
                        BlocKind::Static => {
                            if (transform.translation().y / BLOC_SIZE) as usize > (*line - index as usize) {
                                transform.move_down(BLOC_SIZE);
                            }
                        },
                        _ => {}
                    }
                }
            }
        }
    }
}
