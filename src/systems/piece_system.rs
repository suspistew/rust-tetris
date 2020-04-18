use crate::bloc::{Bloc, BlocKind, BLOC_SIZE};
use crate::piece::{Piece, PieceSystemState};
use crate::tetris::{TetrisResource, MOVEMENT_DELAY};
use amethyst::assets::Handle;
use amethyst::core::components::Transform;
use amethyst::core::timing::Time;
use amethyst::derive::SystemDesc;
use amethyst::ecs::{
    prelude::Entities, Join, Read, ReadStorage, System, SystemData, Write, WriteStorage,
};
use amethyst::renderer::{SpriteRender, SpriteSheet};
use rand::{thread_rng, Rng};

#[derive(SystemDesc)]
pub struct PieceSystem {
    #[system_desc(skip)]
    active_piece: Piece,
    #[system_desc(skip)]
    movement_timer: Option<f32>,
}

impl PieceSystem {
    pub fn new() -> PieceSystem {
        PieceSystem {
            active_piece: Piece::random_new(),
            movement_timer: Some(MOVEMENT_DELAY),
        }
    }

    pub fn switch_to_next_piece(&mut self) {
        self.active_piece = Piece::random_new();
    }
}

impl<'s> System<'s> for PieceSystem {
    type SystemData = (
        WriteStorage<'s, Bloc>,
        WriteStorage<'s, Transform>,
        Read<'s, Time>,
        Write<'s, TetrisResource>,
        WriteStorage<'s, SpriteRender>,
        Entities<'s>,
    );

    fn run(
        &mut self,
        (mut allblocs, mut transforms, time, mut tetris_resource, mut sprite_renders, entities): Self::SystemData,
    ) {
        if let Some(mut timer) = self.movement_timer {
            timer -= time.delta_seconds();
            if timer <= 0.0 || tetris_resource.movement_timer < timer {
                match tetris_resource.piece_state {
                    PieceSystemState::WAITING => {
                        tetris_resource.piece_state = PieceSystemState::MOVING(5, 22);
                        let sprite_sheet_handle =
                            tetris_resource.sprite_sheet_handle.as_ref().unwrap();
                        self.switch_to_next_piece();
                        let offsets = self.active_piece.get_current_offsets();
                        let color: usize = thread_rng().gen_range(1, 10);
                        for offset in offsets {
                            let mut t = Transform::default();
                            t.set_translation_xyz(
                                5.0 * BLOC_SIZE + offset.0 * BLOC_SIZE,
                                20.0 * BLOC_SIZE + offset.1 * BLOC_SIZE,
                                0.0,
                            );
                            entities
                                .build_entity()
                                .with(
                                    init_new_bloc_sprite_render(sprite_sheet_handle.clone(), color),
                                    &mut sprite_renders,
                                )
                                .with(t, &mut transforms)
                                .with(Bloc::new(BlocKind::Moving), &mut allblocs)
                                .build();
                        }
                    }
                    PieceSystemState::MOVING(x, y) => {
                        let mut static_values: Vec<(u32, u32)> = Vec::new();
                        let mut piece_values: Vec<(u32, u32)> = Vec::new();
                        for (bloc, mut transform) in (&mut allblocs, &mut transforms).join() {
                            let t = (
                                (transform.translation().x / BLOC_SIZE) as u32,
                                (transform.translation().y / BLOC_SIZE) as u32,
                            );
                            match bloc.kind {
                                BlocKind::Moving => piece_values.push(t),
                                _ => static_values.push(t),
                            };
                        }

                        let should_move_piece = {
                            let mut res = true;
                            for (x, y) in piece_values.iter() {
                                for (xx, yy) in static_values.iter() {
                                    if x == xx && y == &(yy + 1) {
                                        res = false;
                                    }
                                }
                            }
                            res
                        };

                        if should_move_piece {
                            for (mut bloc, mut transform) in (&mut allblocs, &mut transforms).join()
                            {
                                match bloc.kind {
                                    BlocKind::Moving => {
                                        transform.move_down(BLOC_SIZE);
                                    }
                                    _ => {}
                                };
                            }
                        } else {
                            for (mut bloc, mut transform) in (&mut allblocs, &mut transforms).join()
                            {
                                match bloc.kind {
                                    BlocKind::Moving => {
                                        bloc.kind = BlocKind::Static;
                                        tetris_resource.piece_state = PieceSystemState::WAITING;
                                    }
                                    _ => {}
                                }
                            }
                        }
                    }
                }
                self.movement_timer.replace(tetris_resource.movement_timer);
            } else {
                self.movement_timer.replace(timer);
            }
        }
    }
}

fn init_new_bloc_sprite_render(sph: Handle<SpriteSheet>, color: usize) -> SpriteRender {
    SpriteRender {
        sprite_sheet: sph,
        sprite_number: color,
    }
}
