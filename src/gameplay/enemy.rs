use avian2d::prelude::*;
use bevy::prelude::*;

use crate::{
    app_state::AppState,
    gameplay::{
        collisions::CollisionLayer,
        energy::{AttackPoints, HitPoints},
        explosion::ExplosionChain,
    },
};

pub const ENEMY_BASE_SIZE: Vec2 = Vec2::new(80., 30.);
pub const ENEMY_DEFENDER_SIZE: Vec2 = Vec2::new(28., 28.);
pub const ENEMY_LAND_SIZE: Vec2 = Vec2::new(1., 1.);
pub const ENEMY_WALL_SIZE: Vec2 = Vec2::new(1., 1.);

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Enemy>()
            .register_type::<EnemyClass>()
            .register_type::<EnemyTeam>()
            .add_observer(on_enemy_collision)
            .add_observer(remove_enemy_when_destroyed)
            .add_observer(spawn_chain_when_destroyed_by_player);
    }
}

#[derive(Component, Clone, Debug, Reflect)]
#[reflect(Component)]
pub struct Enemy;

#[derive(Component, Clone, Debug, PartialEq, Eq, Reflect)]
#[reflect(Component)]
pub enum EnemyClass {
    Base,
    Defender,
    Land,
    Wall,
    // Projectile,
}

impl std::fmt::Display for EnemyClass {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EnemyClass::Base => write!(f, "Base"),
            EnemyClass::Defender => write!(f, "Defender"),
            EnemyClass::Land => write!(f, "Land"),
            EnemyClass::Wall => write!(f, "Wall"),
            // TODO: Add projectiles (and probably Attacker?)
            // EnemyClass::Projectile => write!(f, "Projectile"),
        }
    }
}

#[derive(Component, Clone, Debug, PartialEq, Eq, Reflect)]
#[reflect(Component)]
pub enum EnemyTeam {
    Alien,
    Demon,
    Ghost,
    Skull,
}

impl std::fmt::Display for EnemyTeam {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EnemyTeam::Alien => write!(f, "Alien"),
            EnemyTeam::Demon => write!(f, "Demon"),
            EnemyTeam::Ghost => write!(f, "Ghost"),
            EnemyTeam::Skull => write!(f, "Skull"),
        }
    }
}

#[derive(Component, Clone, Debug, PartialEq, Eq, Reflect)]
#[reflect(Component)]
pub enum EnemyDestruction {
    Required,
    Impossible,
}

#[derive(Bundle)]
pub struct EnemyBundle {
    enemy: Enemy,
    class: EnemyClass,
    team: EnemyTeam,
    destruction: EnemyDestruction,
    name: Name,
    scene: SceneRoot,
    ap: AttackPoints,
    hp: HitPoints,
    transform: Transform,
    rigid_body: RigidBody,
    collider: Collider,
    collision_events_enabled: CollisionEventsEnabled,
    collision_layers: CollisionLayers,

    // TODO: specify scope outside of file to reduce coupling and potentially put Debug derive back on EnemyBundle and LevelConfig
    state_scoped: StateScoped<AppState>,
}

impl EnemyBundle {
    pub fn new_base(asset_server: &AssetServer, position: Vec2) -> Self {
        Self {
            enemy: Enemy,
            name: Name::new("Alien Base"),
            team: EnemyTeam::Alien,
            class: EnemyClass::Base,
            // TODO: this should be a marker trait for simplicity and querying, but right now everything is an EnemyBundle
            destruction: EnemyDestruction::Required,
            scene: SceneRoot(
                asset_server
                    .load(GltfAssetLabel::Scene(0).from_asset("enemies/enemy-one-base.glb")),
            ),
            ap: AttackPoints(1),
            hp: HitPoints(3),
            transform: Transform::from_xyz(position.x, position.y, 2.),
            rigid_body: RigidBody::Static,
            collider: Collider::rectangle(ENEMY_BASE_SIZE.x, ENEMY_BASE_SIZE.y),
            collision_events_enabled: CollisionEventsEnabled,
            collision_layers: CollisionLayers::new(
                CollisionLayer::EnemyBase,
                [CollisionLayer::Player, CollisionLayer::PlayerProjectile],
            ),
            state_scoped: StateScoped(AppState::Gameplay),
        }
    }
    pub fn new_defender(asset_server: &AssetServer, position: Vec2) -> Self {
        Self {
            enemy: Enemy,
            name: Name::new("Alien Defender"),
            team: EnemyTeam::Alien,
            class: EnemyClass::Defender,
            destruction: EnemyDestruction::Required,
            scene: SceneRoot(
                asset_server.load(GltfAssetLabel::Scene(0).from_asset("enemies/enemy-one.glb")),
            ),
            ap: AttackPoints(1),
            hp: HitPoints(1),
            transform: Transform::from_xyz(position.x, position.y, 3.),
            rigid_body: RigidBody::Static,
            collider: Collider::rectangle(ENEMY_DEFENDER_SIZE.x, ENEMY_DEFENDER_SIZE.y),
            collision_events_enabled: CollisionEventsEnabled,
            collision_layers: CollisionLayers::new(
                CollisionLayer::EnemyDefender,
                [CollisionLayer::Player, CollisionLayer::PlayerProjectile],
            ),
            state_scoped: StateScoped(AppState::Gameplay),
        }
    }
    pub fn new_land(asset_server: &AssetServer, position: Vec2, scale: Vec2) -> Self {
        Self {
            enemy: Enemy,
            name: Name::new("Alien Land"),
            team: EnemyTeam::Alien,
            class: EnemyClass::Land,
            destruction: EnemyDestruction::Required,
            scene: SceneRoot(
                asset_server.load(GltfAssetLabel::Scene(0).from_asset("enemies/enemy-land.glb")),
            ),
            ap: AttackPoints(1),
            hp: HitPoints(1),
            transform: Transform::from_xyz(position.x, position.y, 1.).with_scale(scale.extend(1.)),
            rigid_body: RigidBody::Static,
            collider: Collider::rectangle(ENEMY_LAND_SIZE.x, ENEMY_LAND_SIZE.y),
            collision_events_enabled: CollisionEventsEnabled,
            collision_layers: CollisionLayers::NONE,
            state_scoped: StateScoped(AppState::Gameplay),
        }
    }
    pub fn new_wall(asset_server: &AssetServer, position: Vec2, scale: Vec2) -> Self {
        Self {
            enemy: Enemy,
            name: Name::new("Alien Wall"),
            team: EnemyTeam::Alien,
            class: EnemyClass::Wall,
            destruction: EnemyDestruction::Impossible,
            scene: SceneRoot(
                asset_server.load(GltfAssetLabel::Scene(0).from_asset("enemies/enemy-wall.glb")),
            ),
            ap: AttackPoints(0),
            hp: HitPoints(0),
            transform: Transform::from_xyz(position.x, position.y, 1.).with_scale(scale.extend(1.)),
            rigid_body: RigidBody::Static,
            collider: Collider::rectangle(ENEMY_WALL_SIZE.x, ENEMY_WALL_SIZE.y),
            collision_events_enabled: CollisionEventsEnabled,
            collision_layers: CollisionLayers::new(
                CollisionLayer::EnemyWall,
                [CollisionLayer::Player, CollisionLayer::PlayerProjectile],
            ),
            state_scoped: StateScoped(AppState::Gameplay),
        }
    }
    pub fn requires_destruction(&self) -> bool {
        self.destruction == EnemyDestruction::Required
    }
}

#[derive(Event, Clone, Debug, Reflect)]
pub struct EnemyCollisionEvent {
    pub attacking_points: AttackPoints,
    pub class: EnemyClass,
    pub team: EnemyTeam,
}

impl EnemyCollisionEvent {
    pub fn new(class: EnemyClass, team: EnemyTeam, attacking_points: AttackPoints) -> Self {
        Self {
            attacking_points,
            class,
            team,
        }
    }
}

fn on_enemy_collision(
    trigger: Trigger<EnemyCollisionEvent>,
    mut commands: Commands,
    mut enemy_q: Query<(&mut HitPoints, &Transform, &EnemyClass, &EnemyTeam)>,
) {
    let Ok((mut hp, transform, class, team)) = enemy_q.get_mut(trigger.target()) else {
        warn!("Could not find just collided Enemy");
        return;
    };
    hp.0 = hp.0.saturating_sub(trigger.event().attacking_points.0);
    if hp.0 == 0 {
        commands.trigger_targets(
            EnemyDestroyedEvent {
                class: class.clone(),
                destruction_source: EnemyDestructionSource::Player,
                position: transform.translation.truncate(),
                scale: transform.scale.truncate(),
                team: team.clone(),
            },
            trigger.target(),
        );
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Reflect)]
pub enum EnemyDestructionSource {
    ExplosionChain,
    Player,
}

#[derive(Event, Clone, Debug, Reflect)]
pub struct EnemyDestroyedEvent {
    pub class: EnemyClass,
    pub destruction_source: EnemyDestructionSource,
    pub position: Vec2,
    pub scale: Vec2,
    pub team: EnemyTeam,
}

fn remove_enemy_when_destroyed(trigger: Trigger<EnemyDestroyedEvent>, mut commands: Commands) {
    commands.entity(trigger.target()).despawn();
}

fn spawn_chain_when_destroyed_by_player(
    trigger: Trigger<EnemyDestroyedEvent>,
    mut commands: Commands,
) {
    let EnemyDestroyedEvent {
        class,
        destruction_source,
        position: _,
        scale: _,
        team,
    } = trigger.event();
    if destruction_source == &EnemyDestructionSource::Player {
        if let Some(next_stage) = ExplosionChain::following_stage(class) {
            commands.spawn((
                StateScoped(AppState::Gameplay),
                ExplosionChain::new(team.clone(), next_stage),
            ));
        }
    }
}
