use crate::bloc::{Bloc, BlocKind, BLOC_SIZE};
use crate::piece::Piece;
use crate::tetris::{TetrisResource, MOVEMENT_DELAY};
use amethyst::core::{SystemDesc, Transform};
use amethyst::derive::SystemDesc;
use amethyst::ecs::{Join, Read, ReadStorage, System, SystemData, World, Write, WriteStorage};
use amethyst::input::{InputHandler, StringBindings};

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
        match input.action_is_down("accelerate") {
            Some(true) => tetris_resource.movement_timer = 0.025,
            _ => tetris_resource.movement_timer = MOVEMENT_DELAY,
        };

        let left = match input.action_is_down("left") {
            Some(true) => true,
            _ => false,
        };
        let right = match input.action_is_down("right") {
            Some(true) => true,
            _ => false,
        };

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

        if self.action_reset && left && !right {
            let mut res = true;
            for (x, y) in piece_values.iter() {
                for (xx, yy) in static_values.iter() {
                    if y == yy && x == &(xx + 1) {
                        res = false
                    }
                }
            }
            if res {
                self.action_reset_timer.replace(0.1);
                for (mut bloc, mut transform) in (&blocs, &mut transforms).join() {
                    match bloc.kind {
                        BlocKind::Moving => {
                            transform.move_left(BLOC_SIZE);
                        }
                        _ => {}
                    };
                }
            }
        } else if self.action_reset && right && !left {
            let mut res = true;
            for (x, y) in piece_values.iter() {
                for (xx, yy) in static_values.iter() {
                    if y == yy && x == &(xx - 1) {
                        res = false
                    }
                }
            }
            if res {
                self.action_reset_timer.replace(0.1);
                for (mut bloc, mut transform) in (&blocs, &mut transforms).join() {
                    match bloc.kind {
                        BlocKind::Moving => {
                            transform.move_right(BLOC_SIZE);
                        }
                        _ => {}
                    };
                }
            }
        }

        if !left && !right {
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
