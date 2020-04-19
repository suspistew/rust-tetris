use crate::bloc::{Bloc, BlocKind, BLOC_SIZE};
use crate::piece::{PieceKind, PieceSystemState};
use crate::tetris::{TetrisResource};
use amethyst::core::{Transform};
use amethyst::derive::SystemDesc;
use amethyst::ecs::{Join, Read, Entities, System, SystemData, Write, WriteStorage};
use amethyst::input::{InputHandler, StringBindings};
use amethyst::renderer::{SpriteRender, SpriteSheet};
use amethyst::assets::Handle;

use amethyst::core::timing::Time;

#[derive(SystemDesc)]
pub struct RotationSystem {
    action_reset: bool,
}

impl RotationSystem {
    pub fn new() -> RotationSystem {
        RotationSystem {
            action_reset: true,
        }
    }

    fn handle_reset_movement(&mut self, rotation: bool) {
        if !rotation {
            self.action_reset = true;
        }else{
            self.action_reset = false;
        }
    }
}

impl<'s> System<'s> for RotationSystem {
    type SystemData = (
        WriteStorage<'s, Transform>,
        WriteStorage<'s, Bloc>,
        Read<'s, InputHandler<StringBindings>>,
        Read<'s, Time>,
        Write<'s, TetrisResource>,
        WriteStorage<'s, SpriteRender>,
        Entities<'s>,
    );

    fn run(&mut self, (mut transforms, mut blocs, input, time, mut tetris_resource, mut sprite_renders, mut entities): Self::SystemData) {
        let rotation = read_action(&input);
        if self.action_reset && rotation {
            let rotation_offsets = {
                let next_orientation = tetris_resource.active_piece.orientation.next_orientation();
                PieceKind::get_offsets(&tetris_resource.active_piece.kind, &next_orientation)
            };
            
            if let PieceSystemState::MOVING(x, y) = tetris_resource.piece_state {
                let mut should_rotate_piece = true;
                for offset in rotation_offsets.iter() {
                    for (bloc, transform) in (&blocs, &mut transforms).join() {
                        match bloc.kind {
                            BlocKind::Moving => {},
                            _ => {
                                let translation = transform.translation();
                                if translation.x / BLOC_SIZE == x as f32 + offset.0 
                                    && translation.y / BLOC_SIZE == y as f32 + offset.1 {
                                        should_rotate_piece = false;
                                        break;
                                    }   
                            }
                        }
                    }
                }

                if should_rotate_piece {
                    for (bloc, transform, entity) in (&blocs, &mut transforms, &*entities).join() {
                        match bloc.kind {
                            BlocKind::Moving => {entities.delete(entity);},
                            _ => {}
                        }
                    }
                    // update piece and state
                    tetris_resource.active_piece.rotate();
                    let offsets = tetris_resource.active_piece.get_current_offsets();
                        let sprite_sheet_handle =
                            tetris_resource.sprite_sheet_handle.as_ref().unwrap();
                        let color: usize = tetris_resource.active_piece.color;
                        for offset in offsets {
                            let mut t = Transform::default();
                            t.set_translation_xyz(
                                x as f32 * BLOC_SIZE + offset.0 * BLOC_SIZE,
                                y as f32 * BLOC_SIZE + offset.1 * BLOC_SIZE,
                                0.0,
                            );
                            entities
                                .build_entity()
                                .with(
                                    init_new_bloc_sprite_render(sprite_sheet_handle.clone(), color),
                                    &mut sprite_renders,
                                )
                                .with(t, &mut transforms)
                                .with(Bloc::new(BlocKind::Moving), &mut blocs)
                                .build();
                        }
                }

            }


        }  
        self.handle_reset_movement(rotation);
    }
}

fn read_action(input: &InputHandler<StringBindings>) -> bool {
    match input.action_is_down("rotate") {
        Some(e) => e,
        _ => false
    }
}

fn init_new_bloc_sprite_render(sph: Handle<SpriteSheet>, color: usize) -> SpriteRender {
    SpriteRender {
        sprite_sheet: sph,
        sprite_number: color,
    }
}