use bevy::prelude::*;

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Enemy>()
            .register_type::<EnemyBase>()
            .register_type::<EnemyOne>()
            .register_type::<EnemyOneBase>()
            .add_systems(Startup, setup);
    }
}

#[derive(Component, Clone, Debug, Reflect)]
#[reflect(Component)]
pub struct EnemyBase;

#[derive(Component, Clone, Debug, Reflect)]
#[reflect(Component)]
pub struct Enemy;

#[derive(Component, Clone, Debug, Reflect)]
#[reflect(Component)]
pub struct EnemyOneBase;

#[derive(Component, Clone, Debug, Reflect)]
#[reflect(Component)]
pub struct EnemyOne;

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        EnemyBase,
        EnemyOneBase,
        Name::new("EnemyOneBase"),
        SceneRoot(
            asset_server.load(GltfAssetLabel::Scene(0).from_asset("enemies/enemy-one-base.glb")),
        ),
        Transform::from_xyz(0., 330., 2.),
    ));
    commands.spawn((
        Enemy,
        EnemyOne,
        Name::new("EnemyOne"),
        SceneRoot(asset_server.load(GltfAssetLabel::Scene(0).from_asset("enemies/enemy-one.glb"))),
        Transform::from_xyz(-60., 200., 3.),
    ));
    commands.spawn((
        Enemy,
        EnemyOne,
        Name::new("EnemyOne"),
        SceneRoot(asset_server.load(GltfAssetLabel::Scene(0).from_asset("enemies/enemy-one.glb"))),
        Transform::from_xyz(-20., 200., 3.),
    ));
}
