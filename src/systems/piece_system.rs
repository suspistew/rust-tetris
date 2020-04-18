use crate::bloc::{Bloc, BlocKind};
use crate::piece::Piece;
use crate::tetris::TetrisResource;
use amethyst::assets::Handle;
use amethyst::core::components::Transform;
use amethyst::core::timing::Time;
use amethyst::derive::SystemDesc;
use amethyst::ecs::{
    prelude::Entities, Join, Read, ReadStorage, System, SystemData, Write, WriteStorage,
};
use amethyst::renderer::{SpriteRender, SpriteSheet};
use rand::{thread_rng, Rng};

const MOVEMENT_DELAY: f32 = 0.1;

enum PieceSystemState {
    MOVING(u32, u32),
    WAITING,
}

#[derive(SystemDesc)]
pub struct PieceSystem {
    #[system_desc(skip)]
    active_piece: Piece,
    #[system_desc(skip)]
    movement_timer: Option<f32>,
    #[system_desc(skip)]
    piece_state: PieceSystemState,
}

impl PieceSystem {
    pub fn new() -> PieceSystem {
        PieceSystem {
            active_piece: Piece::random_new(),
            movement_timer: Some(MOVEMENT_DELAY),
            piece_state: PieceSystemState::WAITING,
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
        Read<'s, TetrisResource>,
        WriteStorage<'s, SpriteRender>,
        Entities<'s>,
    );

    fn run(
        &mut self,
        (mut allblocs, mut transforms, time, tetris_resource, mut sprite_renders, entities): Self::SystemData,
    ) {
        if let Some(mut timer) = self.movement_timer {
            timer -= time.delta_seconds();
            if timer <= 0.0 {
                match self.piece_state {
                    PieceSystemState::WAITING => {
                        let sprite_sheet_handle =
                            tetris_resource.sprite_sheet_handle.as_ref().unwrap();
                        self.switch_to_next_piece();
                        self.piece_state = PieceSystemState::MOVING(5, 22);
                        let offsets = self.active_piece.get_current_offsets();
                        let color: usize = thread_rng().gen_range(1, 10);
                        for offset in offsets {
                            let mut t = Transform::default();
                            t.set_translation_xyz(
                                5.0 * 36.0 + offset.0 * 36.0,
                                20.0 * 36.0 + offset.1 * 36.0,
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
                                (transform.translation().x / 36.0) as u32,
                                (transform.translation().y / 36.0) as u32,
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
                                        transform.move_down(36.0);
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
                                        self.piece_state = PieceSystemState::WAITING;
                                    }
                                    _ => {}
                                }
                            }
                        }
                    }
                }
                self.movement_timer.replace(MOVEMENT_DELAY);
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
