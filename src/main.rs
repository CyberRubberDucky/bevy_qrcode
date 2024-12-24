use bevy::prelude::*;
use bevy::color::palettes::css::{BLACK, WHITE};
use qrcode::QrCode;

#[derive(Component)]
struct DataBlock {
    color: Color,
    position: Vec3,
}

#[derive(Resource)]
struct QrCodePattern {
    pattern: String,
    width: usize,
    height: usize,
}

fn main() {
    let code = QrCode::new(b"Merry christmas NERDS!").unwrap();
    let pattern = code
        .render::<char>()
        .dark_color('#')
        .quiet_zone(false)
        .module_dimensions(1, 1)
        .build();

    let width = pattern.lines().next().unwrap_or("").len();
    let height = pattern.lines().count();

    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(QrCodePattern {
            pattern,
            width,
            height,
        })
        .add_systems(Startup, (spawn_pattern, add_center_image))
        .insert_resource(ClearColor(Color::rgb(1.0, 1.0, 1.0)))
        .run();
}

fn spawn_pattern(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    qr_code_pattern: Res<QrCodePattern>,
) {
    commands.spawn(Camera2d);

    let block_size = 10.0;

    let total_width = qr_code_pattern.width as f32 * block_size;
    let total_height = qr_code_pattern.height as f32 * block_size;
    let offset_x = -total_width / 2.0;
    let offset_y = total_height / 2.0;

    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::WHITE,
                custom_size: Some(Vec2::new(total_width, total_height)),
                ..default()
            },
            transform: Transform::from_translation(Vec3::new(0.0, 0.0, -1.0)),
            ..default()
        },
    ));

    let center_size = 70;
    let corner_size = 7;

    let center_left = (qr_code_pattern.width / 2) - (center_size as usize / block_size as usize / 2);
    let center_right = center_left + (center_size as usize / block_size as usize);
    let center_top = (qr_code_pattern.height / 2) - (center_size as usize / block_size as usize / 2);
    let center_bottom = center_top + (center_size as usize / block_size as usize);

    let mut row = 0;

    for line in qr_code_pattern.pattern.lines() {
        let mut col = 0;
        for ch in line.chars() {
            let color = if ch == '#' { Color::BLACK } else { Color::WHITE };

            let position = Vec3::new(
                offset_x + col as f32 * block_size,
                offset_y - row as f32 * block_size,
                0.0,
            );

            let in_center = row >= center_top
                && row < center_bottom
                && col >= center_left
                && col < center_right;

            let is_corner = (row < corner_size && col < corner_size)
                || (row < corner_size && col >= qr_code_pattern.width - corner_size)
                || (row >= qr_code_pattern.height - corner_size && col < corner_size);

            if !in_center {
                if is_corner {
                    spawn_corner_block(
                        &mut commands,
                        &mut meshes,
                        &mut materials,
                        color,
                        position,
                        block_size,
                    );
                } else {
                    spawn_circle_data_block(
                        &mut commands,
                        &mut meshes,
                        &mut materials,
                        color,
                        position,
                        block_size,
                    );
                }
            }

            col += 1;
        }
        row += 1;
    }
}

fn add_center_image(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((Node{
        width: Val::Px(50.0),
        height: Val::Px(50.0),
        left: Val::Px(620.0),
        top: Val::Px(400.0),
        ..default()
    },
        ImageNode::new(asset_server.load("assets/myface.png")),
    ));
}



fn spawn_circle_data_block(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
    color: Color,
    position: Vec3,
    block_size: f32,
) {
    commands.spawn((
        DataBlock { color, position },
        Mesh2d(meshes.add(Circle::new(block_size / 2.0))),
        MeshMaterial2d(materials.add(color)),
        Transform::from_translation(position),
    ));
}

fn spawn_corner_block(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
    color: Color,
    position: Vec3,
    block_size: f32,
) {
    commands.spawn((
        DataBlock { color, position },
        Mesh2d(meshes.add(Rectangle::default())),
        MeshMaterial2d(materials.add(color)),
        Transform::from_translation(position).with_scale(Vec3::new(block_size, block_size, 1.0)),
    ));
}