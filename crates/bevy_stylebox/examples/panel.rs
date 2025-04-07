use bevy::prelude::*;
use bevy_stylebox::*;

fn main() {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins)
        .add_plugins(StyleboxPlugin)
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2d::default());
    commands.spawn((
                       Stylebox {
            slice: UiRect::all(Val::Px(16.)),
            texture: asset_server.load("panel-blue.png"),
            ..default()
        },
        Node {
            position_type: PositionType::Absolute,
            top: Val::Percent(25.),
            bottom: Val::Percent(25.),
            left: Val::Percent(25.),
            right: Val::Percent(25.),
            ..default()
        },

    ));
}
