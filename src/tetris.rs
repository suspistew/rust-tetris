use crate::bloc::{Bloc, BlocKind, BLOC_SIZE};
use crate::piece::{PieceSystemState, Piece};
use amethyst::{
    assets::{AssetStorage, Handle, Loader},
    core::transform::Transform,
    prelude::*,
    renderer::{Camera, ImageFormat, SpriteRender, SpriteSheet, SpriteSheetFormat, Texture},
};

use rand::{thread_rng, Rng};

pub struct Tetris;

pub const MOVEMENT_DELAY: f32 = 0.3;

pub struct TetrisResource {
    pub sprite_sheet_handle: Option<Handle<SpriteSheet>>,
    pub movement_timer: f32,
    pub piece_state: PieceSystemState,
    pub active_piece: Piece,
}

impl TetrisResource {
    fn new(sph: Option<Handle<SpriteSheet>>) -> TetrisResource {
        TetrisResource {
            sprite_sheet_handle: sph,
            movement_timer: MOVEMENT_DELAY,
            piece_state: PieceSystemState::WAITING,
            active_piece: Piece::random_new(thread_rng().gen_range(1, 10)),
        }
    }

    pub fn switch_to_next_piece(&mut self) {
        self.active_piece = Piece::random_new(thread_rng().gen_range(1, 10));
    }
}

impl Default for TetrisResource {
    fn default() -> Self {
        TetrisResource {
            sprite_sheet_handle: None,
            movement_timer: MOVEMENT_DELAY,
            piece_state: PieceSystemState::WAITING,
            active_piece: Piece::random_new(thread_rng().gen_range(1, 10)),
        }
    }
}

impl SimpleState for Tetris {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let mut world = data.world;
        let tetris_resource = load_tetris_resource(world);
        world.insert(tetris_resource);
        initialize_board_borders(world);
        initialize_camera(world);
    }
}

fn load_tetris_resource(world: &mut World) -> TetrisResource {
    let texture_handle = {
        let asset_loader = world.read_resource::<Loader>();
        let texture_storage = world.read_resource::<AssetStorage<Texture>>();
        asset_loader.load(
            "textures/blocs.png",
            ImageFormat::default(),
            (),
            &texture_storage,
        )
    };

    let asset_loader = world.read_resource::<Loader>();
    let sprite_sheet_store = world.read_resource::<AssetStorage<SpriteSheet>>();
    let sprite_sheet_handle = asset_loader.load(
        "textures/blocs.ron",
        SpriteSheetFormat(texture_handle),
        (),
        &sprite_sheet_store,
    );
    TetrisResource::new(Some(sprite_sheet_handle))
}

fn initialize_board_borders(world: &mut World) {
    let sprite_sheet_handle = world
        .read_resource::<TetrisResource>()
        .sprite_sheet_handle
        .as_ref()
        .unwrap()
        .clone();

    let sprite_render = SpriteRender {
        sprite_sheet: sprite_sheet_handle.clone(),
        sprite_number: 1,
    };

    for y in 0..22 {
        initialize_bloc(0.0, y as f32 * BLOC_SIZE, world, sprite_render.clone());
        initialize_bloc(
            11.0 * BLOC_SIZE,
            y as f32 * BLOC_SIZE,
            world,
            sprite_render.clone(),
        );
        if y == 0 {
            for x in 1..11 {
                initialize_bloc(
                    x as f32 * BLOC_SIZE,
                    y as f32 * BLOC_SIZE,
                    world,
                    sprite_render.clone(),
                );
            }
        }
    }
}

fn initialize_bloc(x: f32, y: f32, world: &mut World, sprite_render: SpriteRender) {
    let mut transform = Transform::default();
    transform.set_translation_xyz(x, y, 0.0);
    world
        .create_entity()
        .with(sprite_render)
        .with(Bloc::new(BlocKind::Border))
        .with(transform)
        .build();
}

fn initialize_camera(world: &mut World) {
    let mut transform = Transform::default();
    transform.set_translation_xyz(216.0 - 18.0, 396.0 - 18.0, 1.0);

    world
        .create_entity()
        .with(Camera::standard_2d(432.0, 792.0))
        .with(transform)
        .build();
}
