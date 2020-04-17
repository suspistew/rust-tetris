use crate::bloc::{Bloc, BlocKind};
use amethyst::{
    assets::{AssetStorage, Handle, Loader},
    core::transform::Transform,
    prelude::*,
    renderer::{Camera, ImageFormat, SpriteRender, SpriteSheet, SpriteSheetFormat, Texture},
};

pub struct Tetris;

impl SimpleState for Tetris {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let mut world = data.world;
        let sprite_sheet_handle = load_sprite_sheet(world);
        initialize_board_borders(world, sprite_sheet_handle);
        initialize_camera(world);
    }
}

fn load_sprite_sheet(world: &mut World) -> Handle<SpriteSheet> {
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
    asset_loader.load(
        "textures/blocs.ron",
        SpriteSheetFormat(texture_handle),
        (),
        &sprite_sheet_store,
    )
}

fn initialize_board_borders(world: &mut World, sprite_sheet_handle: Handle<SpriteSheet>) {
    let sprite_render = SpriteRender {
        sprite_sheet: sprite_sheet_handle.clone(),
        sprite_number: 1,
    };
    for y in 0..22 {
        initialize_bloc(0.0, y as f32 * 36.0, world, sprite_render.clone());
        initialize_bloc(11.0 * 36.0, y as f32 * 36.0, world, sprite_render.clone());
        if y == 0 {
            for x in 1..11 {
                initialize_bloc(
                    x as f32 * 36.0,
                    y as f32 * 36.0,
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
