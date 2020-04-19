use crate::bloc::{Bloc, BlocKind, BLOC_SIZE};
use crate::piece::PieceSystemState;
use crate::tetris::{TetrisResource, MOVEMENT_DELAY};
use amethyst::core::{SystemDesc, Transform};
use amethyst::derive::SystemDesc;
use amethyst::ecs::{Join, Read, Storage, join::JoinIter, ReadStorage, System, SystemData, World, Write, WriteStorage};
use amethyst::input::{InputHandler, StringBindings};
use amethyst::ecs::prelude::DenseVecStorage;

use amethyst::core::timing::Time;

#[derive(SystemDesc)]
pub struct MoveSystem {
    action_reset: bool,
    action_reset_timer: Option<f32>,
}

impl MoveSystem {
    pub fn new() -> MoveSystem {
        MoveSystem {
            action_reset: true,
            action_reset_timer: None,
        }
    }

    fn handle_reset_movement(&mut self, movement: i8, time: &Time) {
        if movement == 0 {
            self.action_reset = true;
            self.action_reset_timer = None;
        } else if let Some(mut timer) = self.action_reset_timer {
            timer -= time.delta_seconds();
            if timer <= 0.0 {
                self.action_reset = true;
                self.action_reset_timer = None;
            } else {
                self.action_reset = false;
                self.action_reset_timer.replace(timer);
            }
        }
    }
}


impl<'s> System<'s> for MoveSystem {
    type SystemData = (
        WriteStorage<'s, Transform>,
        ReadStorage<'s, Bloc>,
        Read<'s, InputHandler<StringBindings>>,
        Read<'s, Time>,
        Write<'s, TetrisResource>,
    );

    fn run(&mut self, (mut transforms, blocs, input, time, mut tetris_resource): Self::SystemData) {
        handle_acceleration(&mut tetris_resource, &input);
        let movement = read_movements_actions(&input);
        if self.action_reset {
            let should_move = { // TODO : find a way to extract this bloc
                let mut res = true;
                let mut static_values: Vec<(i32, i32)> = Vec::new();
                let mut piece_values: Vec<(i32, i32)> = Vec::new();
                for (bloc, mut transform) in (&blocs, &mut transforms).join() {
                    let t = (
                        (transform.translation().x / BLOC_SIZE) as i32,
                        (transform.translation().y / BLOC_SIZE) as i32,
                    );
                    match bloc.kind {
                        BlocKind::Moving => piece_values.push(t),
                        _ => static_values.push(t),
                    };
                }
            
                for (x, y) in piece_values.iter() {
                    for (xx, yy) in static_values.iter() {
                        if y == yy && x == &(xx - movement as i32) {
                            res = false;
                            break;
                        }
                    }
                }
                res
            };

            if should_move {
                self.action_reset_timer.replace(0.1);
                if let PieceSystemState::MOVING(x, y) = tetris_resource.piece_state {
                    tetris_resource.piece_state = PieceSystemState::MOVING((x as i32 + movement as i32) as u32, y);
                };
                for (bloc, transform) in (&blocs, &mut transforms).join() {
                    match bloc.kind {
                        BlocKind::Moving => {
                            transform.prepend_translation_x(movement as f32 * BLOC_SIZE);
                        }
                        _ => {}
                    };
                }
            }
        }
        self.handle_reset_movement(movement, &time);
    }
}

fn handle_acceleration(tetris_resource: &mut TetrisResource, input: &InputHandler<StringBindings>) {
        match input.action_is_down("accelerate") {
            Some(true) => tetris_resource.movement_timer = 0.025,
            _ => tetris_resource.movement_timer = MOVEMENT_DELAY,
        };
}

fn read_movements_actions(input: &InputHandler<StringBindings>) -> i8 {
        ({ if let Some(true) = input.action_is_down("left") { -1 } else { 0 } }) +
        ({ if let Some(true) = input.action_is_down("right") { 1 } else { 0 } })
}