use avian2d::prelude::*;
use bevy::{prelude::*, window::WindowResolution};
use bevy_ecs_ldtk::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;

fn main() {
    println!("setting up app environment");
    let mut app = App::new();

    app.add_plugins((
        DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Aspen Halls".into(),
                name: Some("Aspen Halls".into()),
                resolution: WindowResolution::new(1920, 1080),
                ..default()
            }),
            ..default()
        }),
        avian2d::PhysicsPlugins::default(),
        bevy_ecs_ldtk::LdtkPlugin,
    ));

    app.add_plugins((
        bevy_inspector_egui::bevy_egui::EguiPlugin::default(),
        avian2d::prelude::PhysicsDebugPlugin,
        WorldInspectorPlugin::default(),
    ));

    app.register_ldtk_entity::<LdtkTeleporter>("Teleporter");
    app.add_systems(Startup, (spawn_camera, spawn_world));

    app.run();
}

fn spawn_camera(mut cmds: Commands) {
    info!("spawning camera");
    cmds.spawn((
        Camera2d,
        Projection::Orthographic(OrthographicProjection {
            scale: 0.5,
            ..OrthographicProjection::default_2d()
        }),
        Transform::from_xyz(1280.0 / 4.0, 720.0 / 4.0, 0.0),
    ));
}

fn spawn_world(mut cmds: Commands, asset_server: Res<AssetServer>) {
    info!("spawning ldtk world");
    cmds.insert_resource(LevelSelection::Identifier("TestingHalls".to_string()));
    cmds.spawn(LdtkWorldBundle {
        ldtk_handle: asset_server.load("levels/aspenhallsLevelSet.ldtk").into(),
        ..default()
    });
}

/// teleporter bundle that binds to `LdtkEntity` instances
#[derive(Bundle, Default, LdtkEntity)]
pub struct LdtkTeleporter {
    /// sensor name
    #[with(name_from_instance)]
    name: Name,
    /// shape of sensor
    #[with(teleporter_collider_from_instance)]
    collision_shape: Collider,
    /// rigidbody of collider
    #[with(static_rigidbody)]
    rigidbody: RigidBody,
    /// marks this collider as a sensor
    sensor_tag: Sensor,
    ///  enables collision events on entity
    collision_events: CollisionEventsEnabled,
}

/// creates `Name` from `EntityInstance.identifier`
pub fn name_from_instance(instance: &EntityInstance) -> Name {
    Name::new(instance.identifier.clone())
}

/// creates default rigidbody for most sensors
pub fn static_rigidbody(_instance: &EntityInstance) -> RigidBody {
    RigidBody::Static
}

pub fn teleporter_collider_from_instance(instance: &EntityInstance) -> Collider {
    Collider::rectangle(instance.width as f32, instance.height as f32)
}
