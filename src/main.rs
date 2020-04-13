use amethyst::{
    core::transform::TransformBundle,
    prelude::*,
	input::{InputBundle, StringBindings},
    renderer::{
        plugins::{RenderFlat2D, RenderToWindow},
        types::DefaultBackend,
        RenderingBundle,
    },
    utils::application_root_dir,
};

mod tetris;
mod bloc;
mod systems;

use crate::tetris::Tetris;

fn main() -> amethyst::Result<()> {
    amethyst::start_logger(Default::default());
    let app_root = application_root_dir()?;

    let assets_dir = app_root.join("assets");
    let display_config_path = app_root.join("config").join("display.ron");

	let binding_path = app_root.join("config").join("bindings.ron");
	let input_bundle =
        InputBundle::<StringBindings>::new().with_bindings_from_file(binding_path)?;

    let game_data = GameDataBuilder::default()
        .with_bundle(
            RenderingBundle::<DefaultBackend>::new()
                .with_plugin(
                    RenderToWindow::from_config_path(display_config_path)?
                        .with_clear([0.0, 0.0, 0.0, 1.0]),
                )
                .with_plugin(RenderFlat2D::default()),
        )?
        .with_bundle(TransformBundle::new())?
		.with_bundle(input_bundle)?
		.with(systems::BlocSystem, "bloc_system", &[]);

    let mut game = Application::new(assets_dir, Tetris, game_data)?;
    game.run();

    Ok(())
}
