use bevy::{prelude::*, ecs::query};

use crate::{player::player_bundles::{Weapons, Player}, common::Velocity};

use super::projectile_bundle::Projectile;

pub struct CombatPlugin;


impl Plugin for CombatPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_system(shoot_weapons)
        .add_system(manage_projectiles);
    }
}


pub fn shoot_weapons(
    time: Res<Time>,
    mut weapons_query: Query<&mut Weapons>,
    commands: Commands,
    asset_server: Res<AssetServer>,
    player_query: Query<(&Transform, &Velocity), With<Player>>

) {
    let weapons = &mut weapons_query.single_mut().0;
    let box_commands = &mut Box::new(commands);
    let box_asset_server = &mut Box::new(asset_server);
    let (player_transform, player_velocity) = player_query.single();

    for weapon in weapons.iter_mut() {
        weapon.try_shoot(time.delta(), box_commands, box_asset_server, player_transform.clone(), player_velocity.clone());
    }
}


pub fn manage_projectiles(
    mut query: Query<(&mut Transform, &Velocity), With<Projectile>>
) {
    for (mut transform, velocity) in query.iter_mut() {
        transform.translation += velocity.0.extend(0.0);
    }

    //TODO: delete projectiles
}

