use bevy::prelude::*;
use bevy::sprite::AlphaMode2d;
use rand::{Rng, SeedableRng};
use crate::config::VisualConfig;
use crate::components::MenuStar;

pub fn spawn_menu_starfield(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
    vcfg: &VisualConfig,
) {
    let mut rng = rand::rngs::StdRng::seed_from_u64(0xDEAD_BEEF_u64);
    let far_mat = materials.add(ColorMaterial {
        color: Color::srgba(1.0, 1.0, 1.0, 1.0),
        alpha_mode: AlphaMode2d::Opaque,
        ..default()
    });
    let near_mat = materials.add(ColorMaterial {
        color: Color::srgba(0.8, 0.85, 1.0, 1.0),
        alpha_mode: AlphaMode2d::Opaque,
        ..default()
    });
    let star_sm = meshes.add(Rectangle::new(2.0, 2.0));
    let star_lg = meshes.add(Rectangle::new(3.0, 3.0));
    let radius = 1200.0;

    for _ in 0..1600 {
        let angle = rng.gen_range(0.0..std::f32::consts::TAU);
        let distance = rng.gen_range(0.0..1.0_f32).sqrt() * radius;
        let x = angle.cos() * distance;
        let y = angle.sin() * distance;
        commands.spawn((
            MenuStar,
            Mesh2d(star_sm.clone()),
            MeshMaterial2d(far_mat.clone()),
            Transform::from_xyz(x, y, vcfg.z_layer.z_stars_far),
        ));
    }
    for _ in 0..600 {
        let angle = rng.gen_range(0.0..std::f32::consts::TAU);
        let distance = rng.gen_range(0.0..1.0_f32).sqrt() * radius;
        let x = angle.cos() * distance;
        let y = angle.sin() * distance;
        commands.spawn((
            MenuStar,
            Mesh2d(star_lg.clone()),
            MeshMaterial2d(near_mat.clone()),
            Transform::from_xyz(x, y, vcfg.z_layer.z_stars_near),
        ));
    }
}

pub fn menu_star_drift_system(
    time: Res<Time>,
    mut star_query: Query<&mut Transform, With<MenuStar>>,
) {
    let drift = Vec2::new(8.0, -3.0) * time.delta_secs();
    let wrap_radius = 1200.0;

    for mut transform in star_query.iter_mut() {
        transform.translation.x += drift.x;
        transform.translation.y += drift.y;

        // Wrap: if star drifts outside the spawn radius, teleport back to opposite edge
        let pos = transform.translation.truncate();
        if pos.length() > wrap_radius {
            transform.translation.x = -pos.x * 0.9;
            transform.translation.y = -pos.y * 0.9;
        }
    }
}
