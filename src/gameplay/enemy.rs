use avian2d::prelude::*;
use bevy::{platform::collections::HashMap, prelude::*};

use crate::{
    app_state::AppState,
    gameplay::{
        collisions::CollisionLayer,
        energy::{AttackPoints, HitPoints},
        explosion::ExplosionChain,
        level::LevelStats,
    },
};

pub const ENEMY_BASE_SIZE: Vec2 = Vec2::new(80., 30.);
pub const ENEMY_DEFENDER_SIZE: Vec2 = Vec2::new(28., 28.);
pub const ENEMY_LAND_SIZE: Vec2 = Vec2::new(1., 1.);
pub const ENEMY_SHADOW_SIZE: Vec2 = Vec2::new(28., 28.);
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

#[derive(Component, Clone, Copy, Debug, Hash, PartialEq, Eq, Reflect)]
#[reflect(Component)]
pub enum EnemyClass {
    Base,
    DefenderOne,
    DefenderTwo,
    DefenderThree,
    Land,
    Shadow,
    Wall,
}

impl EnemyClass {
    pub fn in_order() -> [Self; 7] {
        [
            Self::Base,
            Self::Shadow,
            Self::DefenderOne,
            Self::DefenderTwo,
            Self::DefenderThree,
            Self::Land,
            Self::Wall,
        ]
    }
    pub fn index_of(&self, wave: Self) -> usize {
        match wave {
            Self::Base => 0,
            Self::Shadow => 1,
            Self::DefenderOne => 2,
            Self::DefenderTwo => 3,
            Self::DefenderThree => 4,
            Self::Land => 5,
            Self::Wall => 6,
        }
    }
}

#[derive(Component, Clone, Copy, Debug, PartialEq, Eq, Reflect)]
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

#[derive(Clone, Debug, Default, Deref, DerefMut)]
pub struct EnemyCounts(HashMap<EnemyClass, u32>);

impl EnemyCounts {
    pub fn started_with_enemy(&self, class: &EnemyClass) -> bool {
        self.0.get(class).copied().unwrap_or_default() > 0
    }
    pub fn increment(&mut self, class: &EnemyClass) {
        *self.0.entry(*class).or_default() += 1;
    }
    pub fn count(&self, class: &EnemyClass) -> u32 {
        self.0.get(class).copied().unwrap_or_default()
    }
    pub fn total(&self) -> u32 {
        EnemyClass::in_order()
            .into_iter()
            .filter(|class| class != &EnemyClass::Wall)
            .map(|c| self.count(&c))
            .sum()
    }
}

impl From<&Vec<EnemyBundle>> for EnemyCounts {
    fn from(enemies: &Vec<EnemyBundle>) -> Self {
        let mut counts = Self::default();
        for enemy in enemies {
            counts.increment(&enemy.class);
        }
        counts
    }
}

#[derive(Component, Clone, Debug, PartialEq, Eq, Reflect)]
#[reflect(Component)]
pub enum EnemyDestruction {
    Required,
    Impossible,
}

enum DefenderClass {
    One,
    Two,
    Three,
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
    fn new_defender(
        asset_server: &AssetServer,
        position: Vec2,
        defender_class: DefenderClass,
    ) -> Self {
        let (class, scene) = match defender_class {
            DefenderClass::One => (EnemyClass::DefenderOne, "enemies/enemy-defender-one.glb"),
            DefenderClass::Two => (EnemyClass::DefenderTwo, "enemies/enemy-defender-two.glb"),
            DefenderClass::Three => (
                EnemyClass::DefenderThree,
                "enemies/enemy-defender-three.glb",
            ),
        };
        Self {
            enemy: Enemy,
            name: Name::new("Alien Defender"),
            team: EnemyTeam::Alien,
            class,
            destruction: EnemyDestruction::Required,
            scene: SceneRoot(asset_server.load(GltfAssetLabel::Scene(0).from_asset(scene))),
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
    pub fn new_primary_defender(asset_server: &AssetServer, position: Vec2) -> Self {
        Self::new_defender(asset_server, position, DefenderClass::One)
    }
    pub fn new_secondary_defender(asset_server: &AssetServer, position: Vec2) -> Self {
        Self::new_defender(asset_server, position, DefenderClass::Two)
    }
    pub fn new_tertiary_defender(asset_server: &AssetServer, position: Vec2) -> Self {
        Self::new_defender(asset_server, position, DefenderClass::Three)
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
    pub fn new_shadow(asset_server: &AssetServer, position: Vec2) -> Self {
        Self {
            enemy: Enemy,
            name: Name::new("Alien Defender"),
            team: EnemyTeam::Alien,
            class: EnemyClass::Shadow,
            destruction: EnemyDestruction::Required,
            scene: SceneRoot(
                asset_server.load(GltfAssetLabel::Scene(0).from_asset("enemies/enemy-shadow.glb")),
            ),
            ap: AttackPoints(1),
            hp: HitPoints(1),
            transform: Transform::from_xyz(position.x, position.y, 3.),
            rigid_body: RigidBody::Static,
            collider: Collider::rectangle(ENEMY_DEFENDER_SIZE.x, ENEMY_DEFENDER_SIZE.y),
            collision_events_enabled: CollisionEventsEnabled,
            collision_layers: CollisionLayers::new(
                CollisionLayer::EnemyShadow,
                [CollisionLayer::Player],
            ),
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
    let Ok((mut hp, transform, &class, &team)) = enemy_q.get_mut(trigger.target()) else {
        warn!("Could not find just collided Enemy");
        return;
    };
    hp.0 = hp.0.saturating_sub(trigger.event().attacking_points.0);
    if hp.0 == 0 {
        commands.trigger_targets(
            EnemyDestroyedEvent {
                class,
                destruction_source: EnemyDestructionSource::Player,
                position: transform.translation.truncate(),
                scale: transform.scale.truncate(),
                team,
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
    level_stats: Single<&LevelStats>,
) {
    let EnemyDestroyedEvent {
        class,
        destruction_source,
        position: _,
        scale: _,
        team,
    } = trigger.event();
    if destruction_source == &EnemyDestructionSource::Player {
        if let Some(next_stage) = ExplosionChain::following_class(class, &level_stats) {
            commands.spawn((
                StateScoped(AppState::Gameplay),
                ExplosionChain::new(*team, next_stage),
            ));
        }
    }
}
