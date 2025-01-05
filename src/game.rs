use bevy::{input::common_conditions::input_just_pressed, prelude::*};
use rand::Rng;

// max random offset of pipes
const OFFSET: usize = 100;

// velocity of pipes and base
const VEL: f32 = -100.0;

// velocity of background
const VEL_BG: f32 = VEL / 5.0;

// gap between upper/lower pipe
const GAP: f32 = 100.0;

// distance between pipes
const DISTANCE: f32 = 150.0;

// number of pipes to generate
const PIPES: usize = 3;

const VEL_FLAP: f32 = 256.0;
const GRAVITY: f32 = -1024.0;

// width/height of various sprites
const W_BG: f32 = 288.0;
const H_BG: f32 = 512.0;
const W_PIPE: f32 = 52.0;
const H_PIPE: f32 = 320.0;
const W_BASE: f32 = 336.0;
const H_BASE: f32 = 112.0;
const W_BIRD: f32 = 34.0;
const H_BIRD: f32 = 24.0;

#[derive(States, Default, Debug, PartialEq, Eq, Hash, Clone)]
pub enum GameState {
    #[default]
    Menu,
    Playing,
}

#[derive(Component)]
pub struct AnimationIndex {
    first: usize,
    count: usize,
}

impl AnimationIndex {
    pub fn new(first: usize, count: usize) -> Self {
        Self { first, count }
    }
}

#[derive(Component, Deref, DerefMut)]
pub struct AnimationTimer(Timer);

impl AnimationTimer {
    pub fn from_seconds(sec: f32) -> Self {
        Self(Timer::from_seconds(sec, TimerMode::Repeating))
    }
}

pub struct LoopSetting {
    threshold: f32,
    offset: f32,
}

impl LoopSetting {
    pub fn new(threshold: f32, offset: f32) -> Self {
        Self { threshold, offset }
    }
}

#[derive(Component)]
pub struct Background(LoopSetting);

#[derive(Component)]
pub struct PipePair(LoopSetting);

#[derive(Component, Deref, DerefMut)]
pub struct Velocity(Vec3);

#[derive(Component, Deref, DerefMut)]
pub struct Acceleration(Vec3);

#[derive(Component)]
pub struct Bird;

#[derive(Event)]
pub struct FlapEvent;

#[derive(Event)]
pub struct GameStartEvent;

#[derive(Event)]
pub struct GameOverEvent;

pub struct FlappyBirdPlugin;

fn setup(
    mut commands: Commands,
    assets: ResMut<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    let mut rng = rand::thread_rng();
    commands.spawn(Camera2d);

    let background = assets.load("./sprites/background-day.png");
    commands.spawn((
        Sprite {
            image: background,
            custom_size: Some(Vec2::new(W_BG * 2.0, H_BG)),
            image_mode: SpriteImageMode::Tiled {
                tile_x: true,
                tile_y: false,
                stretch_value: 1.0,
            },
            ..Default::default()
        },
        Velocity(Vec3::new(VEL_BG, 0.0, 0.0)),
        Background(LoopSetting::new(-W_BG / 2.0, W_BG / 2.0)),
    ));

    let pipe = assets.load("./sprites/pipe.png");
    for i in 0..PIPES {
        let x = i as f32 * DISTANCE + W_BG;
        let y = OFFSET as f32 + H_BASE / 2.0 - rng.gen_range(0..2 * OFFSET) as f32;
        commands
            .spawn((
                Transform::from_translation(Vec3::new(x, y, 0.1)),
                Velocity(Vec3::new(VEL, 0.0, 0.0)),
                PipePair(LoopSetting::new(
                    -(W_BG + W_PIPE) / 2.0,
                    PIPES as f32 * DISTANCE,
                )),
            ))
            .with_children(|parent| {
                parent.spawn((
                    Sprite {
                        image: pipe.clone(),
                        ..Default::default()
                    },
                    Transform::from_translation(Vec3::new(0.0, -(H_PIPE + GAP) / 2.0, 0.0)),
                ));
                parent.spawn((
                    Sprite {
                        image: pipe.clone(),
                        flip_y: true,
                        ..Default::default()
                    },
                    Transform::from_translation(Vec3::new(0.0, (H_PIPE + GAP) / 2.0, 0.0)),
                ));
            });
    }

    let base = assets.load("./sprites/base.png");
    commands.spawn((
        Sprite {
            image: base,
            ..Default::default()
        },
        Transform::from_translation(Vec3::new(0.0, -(H_BG - H_BASE) / 2.0, 0.2)),
        Velocity(Vec3::new(VEL, 0.0, 0.0)),
        Background(LoopSetting::new((W_BG - W_BASE) / 2.0, 0.0)),
    ));

    let bird = assets.load("./sprites/bluebird2.png");
    let bird_layout = TextureAtlasLayout::from_grid(UVec2 { x: 34, y: 24 }, 3, 1, None, None);
    let bird_layout = texture_atlas_layouts.add(bird_layout);
    let animation_index = AnimationIndex::new(0, 3);
    commands.spawn((
        Sprite {
            image: bird,
            texture_atlas: Some(TextureAtlas {
                layout: bird_layout,
                index: animation_index.first,
            }),
            ..Default::default()
        },
        Transform::from_translation(Vec3::new(0.0, H_BASE / 2.0, 1.0)),
        animation_index,
        AnimationTimer::from_seconds(0.2),
        Velocity(Vec3::new(0.0, VEL_FLAP, 0.0)),
        Acceleration(Vec3::new(0.0, GRAVITY, 0.0)),
        Bird,
    ));

    // let numbers = assets.load("./sprites/numbers.png");
    // let numbers_layout =
    //     TextureAtlasLayout::from_grid(UVec2 { x: 24, y: 36 }, 1, 10, None, None);
    // let numbers_layout = texture_atlas_layouts.add(numbers_layout);
    // let animation_index = AnimationIndex {
    //     first: 0,
    //     count: 10,
    // };
    // commands.spawn((
    //     Sprite::from_atlas_image(
    //         numbers,
    //         TextureAtlas {
    //             layout: numbers_layout,
    //             index: animation_index.first,
    //         },
    //     ),
    //     Transform::from_translation(Vec3::new(-100.0, -100.0, 0.2)),
    //     animation_index,
    //     AnimationTimer(Timer::from_seconds(1.0, TimerMode::Repeating)),
    // ));
}

fn animate(time: Res<Time>, mut query: Query<(&AnimationIndex, &mut AnimationTimer, &mut Sprite)>) {
    let delta = time.delta();
    for (index, mut timer, mut sprite) in &mut query {
        if timer.tick(delta).just_finished() {
            if let Some(ref mut atlas) = sprite.texture_atlas {
                atlas.index = (atlas.index + 1 - index.first) % index.count + index.first;
            }
        }
    }
}

fn send_flap(mut event: EventWriter<FlapEvent>) {
    event.send(FlapEvent);
}

fn flap(mut event: EventReader<FlapEvent>, mut query: Query<&mut Velocity, With<Bird>>) {
    for _ in event.read() {
        for mut vel in &mut query {
            vel.0.y = VEL_FLAP;
        }
    }
}

fn update_position(time: Res<Time>, mut query: Query<(&mut Transform, &Velocity)>) {
    for (mut transform, velocity) in &mut query {
        let distance = **velocity * time.delta_secs();
        transform.translation = transform.translation.mul_add(Vec3::ONE, distance);
    }
}

fn update_velocity(time: Res<Time>, mut query: Query<(&mut Velocity, &Acceleration)>) {
    for (mut vel, accel) in &mut query {
        // TODO: update bird angle here?
        let delta = **accel * time.delta_secs();
        vel.0 += delta;
    }
}

fn check_collision(
    pipes: Query<&Transform, With<PipePair>>,
    birds: Query<&Transform, With<Bird>>,
    mut event: EventWriter<GameOverEvent>,
) {
    for bird in &birds {
        if bird.translation.y <= -(H_BG / 2.0 - H_BASE - H_BIRD / 2.0) {
            event.send(GameOverEvent);
        }
        for pipe in &pipes {
            let align_with_pipe = pipe.translation.x - W_PIPE / 2.0 - W_BIRD / 2.0
                < bird.translation.x
                && bird.translation.x < pipe.translation.x + W_PIPE / 2.0 + W_BIRD / 2.0;
            let inside_gap = pipe.translation.y - GAP / 2.0 + H_BIRD / 2.0 < bird.translation.y
                && bird.translation.y < pipe.translation.y + GAP / 2.0 - H_BIRD / 2.0;
            if align_with_pipe && !inside_gap {
                event.send(GameOverEvent);
            }
        }
    }
}

fn loop_background(mut query: Query<(&mut Transform, &Background)>) {
    for (mut transform, &Background(LoopSetting { threshold, offset })) in &mut query {
        if transform.translation.x < threshold {
            transform.translation.x = offset;
        }
    }
}

fn loop_pipes(mut query: Query<(&mut Transform, &PipePair)>) {
    let mut rng = rand::thread_rng();
    for (mut transform, &PipePair(LoopSetting { threshold, offset })) in &mut query {
        if transform.translation.x < threshold {
            let y = (156 - rng.gen_range(0..200)) as f32;
            transform.translation.y = y;
            transform.translation.x += offset;
        }
    }
}

fn send_start(mut event: EventWriter<GameStartEvent>) {
    event.send(GameStartEvent);
}

fn reset_bird(
    event: EventReader<GameStartEvent>,
    mut query: Query<(&mut Transform, &mut Velocity), With<Bird>>,
) {
    if !event.is_empty() {
        for (mut transform, mut vel) in &mut query {
            transform.translation = Vec3::new(0.0, 0.0 + H_BASE / 2.0, 1.0);
            vel.0 = Vec3::new(0.0, VEL_FLAP, 0.0)
        }
    }
}

fn reset_pipes(
    event: EventReader<GameStartEvent>,
    mut query: Query<&mut Transform, With<PipePair>>,
) {
    if !event.is_empty() {
        let mut x = W_BG;
        for mut transform in &mut query {
            transform.translation.x = x;
            x += DISTANCE;
        }
    }
}

fn start(mut event: EventReader<GameStartEvent>, mut state: ResMut<NextState<GameState>>) {
    for _ in event.read() {
        state.set(GameState::Playing);
    }
}

fn game_over(mut event: EventReader<GameOverEvent>, mut state: ResMut<NextState<GameState>>) {
    for _ in event.read() {
        state.set(GameState::Menu);
    }
}

impl Plugin for FlappyBirdPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<GameState>()
            .add_systems(Startup, setup)
            .add_systems(Update, animate)
            .add_systems(
                Update,
                (
                    send_start.run_if(input_just_pressed(KeyCode::Space)),
                    reset_bird,
                    reset_pipes,
                    start,
                )
                    .chain()
                    .run_if(in_state(GameState::Menu)),
            )
            .add_systems(
                FixedUpdate,
                (
                    update_position,
                    update_velocity,
                    loop_background,
                    loop_pipes,
                    flap,
                    check_collision,
                    game_over,
                    send_flap.run_if(input_just_pressed(KeyCode::Space)),
                )
                    .chain()
                    .run_if(in_state(GameState::Playing)),
            )
            .add_event::<GameStartEvent>()
            .add_event::<GameOverEvent>()
            .add_event::<FlapEvent>();
    }
}
