use bevy::prelude::*;
use wasm_bindgen::prelude::*;

mod breakout;

#[wasm_bindgen]
pub fn run() {
    // animation_run()
    breakout::breakout_run()
}

fn animation_run() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest())) // prevents blurry sprites
        .add_systems(Startup, setup)
        .add_systems(Update, animate_sprite)
        .run();
}

#[derive(Component)]
struct AnimationIndices {
    first: usize,
    last: usize,
}

#[derive(Component, Deref, DerefMut)]
struct AnimationTimer(Timer);

fn animate_sprite(
    time: Res<Time>,
    mut query: Query<(&AnimationIndices, &mut AnimationTimer, &mut TextureAtlas)>,
) {
    for (indices, mut timer, mut atlas) in &mut query {
        timer.tick(time.delta());
        if timer.just_finished() {
            atlas.index = if atlas.index == indices.last {
                indices.first
            } else {
                atlas.index + 1
            };
        }
    }
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    let texture = asset_server.load("textures/rpg/chars/gabe/gabe-idle-run.png");
    let layout = TextureAtlasLayout::from_grid(Vec2::new(24.0, 24.0), 7, 1, None, None);
    let texture_atlas_layout = texture_atlas_layouts.add(layout);
    // Use only the subset of sprites in the sheet that make up the run animation
    let animation_indices = AnimationIndices { first: 1, last: 6 };
    commands.spawn(Camera2dBundle::default());
    commands.spawn((
        SpriteSheetBundle {
            texture,
            atlas: TextureAtlas {
                layout: texture_atlas_layout,
                index: animation_indices.first,
            },
            transform: Transform::from_scale(Vec3::splat(6.0)),
            ..default()
        },
        animation_indices,
        AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),
    ));
}
