use bevy::prelude::*;

const SHIP_START_Y: f32 = 0.0;
const SHIP_START_X: f32 = 0.0;
const SHIP_START_VELOCITY_X: f32 = 500.0;

// mass in kg
const LANDER_MASS: f32 = 15_000.0;
// primary fuel mass for main engine
const LANDER_FUEL_MASS: f32 = 5_000.0;
// main engine maximum thrust in newtons
const LANDER_THRUST_MAX: f32 = 33_750.0;
// main engine minimum thrust in newtons
const LANDER_THRUST_MIN: f32 = 4_500.0;
// gravity in m/s2
const LUNAR_GRAVITY: f32 = 1.625;
const EUROPA_GRAVITY: f32 = 1.314;
const IO_GRAVITY: f32 = 1.796;
const CALLISTO_GRAVITY: f32 = 1.235;
const GANYMEDE_GRAVITY: f32 = 1.428;

const RCS_STEP: f32 = 0.523598775598299;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Update, bevy::window::close_on_esc)
        .add_systems(Startup, setup)
        .add_systems(FixedUpdate, (apply_thrust, apply_velocity))
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>,
mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,) {

    let texture = asset_server.load("lander.png");
    let texture_layout = TextureAtlasLayout::from_grid(Vec2{x: 64.0, y: 60.0}, 6,1, None, None);
    let texture_atlas_layout = texture_atlas_layouts.add(texture_layout);
    // Camera
    commands.spawn(Camera2dBundle::default());
    // ship
    commands.spawn((
        SpriteBundle {
            transform: Transform {
                rotation: Quat::from_rotation_z((30.0_f32).to_radians()),
                translation: Vec3::new(SHIP_START_X, SHIP_START_Y, 0.0),
                ..default()
            },
            texture,
            ..default()
        },
        TextureAtlas {
            layout: texture_atlas_layout,
            index: 0,

        },
        Ship,
        Rcs(Thruster::None),
        Velocity(Vec3::new(0.0, 0.0, 0.0)),
    ));
}

fn apply_thrust(
    time: Res<Time>,
    input: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&mut Transform, &mut Rcs, &mut TextureAtlas)>,
) {
    let mut ship = query.single_mut();
    
    let mut texture_atlas = ship.2;
    let mut rcs = ship.1;
    let mut delta_angle = 0.0; 
    if input.pressed(KeyCode::ArrowLeft) {
        texture_atlas.index = 4;
        delta_angle += time.delta_seconds() * RCS_STEP;
        rcs.0 = Thruster::Left;
    } else if input.pressed(KeyCode::ArrowRight) {
        texture_atlas.index = 2;
        rcs.0 = Thruster::Right;
        delta_angle -= time.delta_seconds() * RCS_STEP;
    } else {
        texture_atlas.index = 0;
        rcs.0 = Thruster::None;
    }
    if input.pressed(KeyCode::ArrowUp) {
        texture_atlas.index += 1;
    }

    ship.0.rotate_z(delta_angle);
}

fn apply_velocity(
    input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut query: Query<(&mut Transform, &mut Velocity)>,
) {
    let mut ship_velocity = query.single_mut();
    let mut delta_velocity = Vec3::new(0.0, 0.0, 0.0);
    
    // caclulate acceleration due to gravity
    let gravity_accel = LUNAR_GRAVITY * 10.0;
    if input.pressed(KeyCode::ArrowUp) {
        delta_velocity.y += 100.0;
    } 
    // muliply the delta velocity by Z rotation to get delta velocity based on
    // off-axis thrust
    delta_velocity = ship_velocity.0.rotation.mul_vec3(delta_velocity);
    // gravity
    delta_velocity.y -= gravity_accel;
     
    ship_velocity.1 .0 += delta_velocity * time.delta_seconds();

    ship_velocity.0.translation.x += ship_velocity.1 .0.x * time.delta_seconds();
    ship_velocity.0.translation.y += ship_velocity.1 .0.y * time.delta_seconds();
}


enum Thruster {
    None,
    Left,
    Right,
}

#[derive(Component)]
struct Ship;

#[derive(Component)]
struct Velocity(Vec3);

#[derive(Component)]
struct Rcs(Thruster);
