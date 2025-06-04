use bevy::prelude::*;

pub struct EnergyPlugin;

impl Plugin for EnergyPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<AttackPoints>()
            .register_type::<HitPoints>();
    }
}

#[derive(Component, Clone, Debug, Reflect)]
#[reflect(Component)]
pub struct AttackPoints(pub u32);

#[derive(Component, Clone, Debug, Reflect)]
#[reflect(Component)]
pub struct HitPoints(pub u32);
