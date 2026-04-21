#import bevy_sprite::mesh2d_vertex_output::VertexOutput

struct StarfieldMaterial {
    camera_pos: vec2<f32>,
    time: f32,
    screen_size: vec2<f32>,
};

@group(2) @binding(0) var<uniform> material: StarfieldMaterial;

fn hash(p: vec2<f32>) -> f32 {
    var p3 = fract(vec3<f32>(p.xyx) * 0.1031);
    p3 = p3 + dot(p3, p3.yzx + 33.33);
    return fract((p3.x + p3.y) * p3.z);
}

fn star_layer(uv: vec2<f32>, cam: vec2<f32>, scale: f32, threshold: f32, brightness: f32) -> f32 {
    let offset_uv = uv + cam / scale;
    let cell = floor(offset_uv * scale);
    let local = fract(offset_uv * scale);
    
    var result = 0.0;
    for (var y = -1; y <= 1; y++) {
        for (var x = -1; x <= 1; x++) {
            let neighbor = cell + vec2<f32>(f32(x), f32(y));
            let h = hash(neighbor);
            if h > threshold {
                let star_pos = vec2<f32>(hash(neighbor + 0.1), hash(neighbor + 0.2));
                let dist = length(local - vec2<f32>(f32(x), f32(y)) - star_pos);
                let twinkle = 0.85 + 0.15 * sin(material.time * (2.0 + h * 3.0) + h * 6.28);
                result += brightness * twinkle * smoothstep(0.02, 0.0, dist);
            }
        }
    }
    return result;
}

@fragment
fn fragment(in: VertexOutput) -> @location(0) vec4<f32> {
    let uv = in.uv;
    let cam = material.camera_pos * 0.0002;  // Much less movement
    
    var stars = 0.0;
    // Far layer - very dense, dim, minimal parallax
    stars += star_layer(uv, cam, 240.0, 0.96, 0.4);
    // Mid-far layer - very dense, dim, minimal parallax
    stars += star_layer(uv, cam * 1.5, 160.0, 0.94, 0.5);
    // Mid layer - dense, dim, minimal parallax
    stars += star_layer(uv, cam * 2.0, 100.0, 0.92, 0.6);
    // Mid-near layer - dense, medium, minimal parallax
    stars += star_layer(uv, cam * 2.5, 50.0, 0.90, 0.7);
    // Near layer - medium, bright, minimal parallax
    stars += star_layer(uv, cam * 3.0, 25.0, 0.88, 0.8);
    // Close layer - sparse, bright, minimal parallax
    stars += star_layer(uv, cam * 4.0, 12.0, 0.85, 1.0);
    
    stars = clamp(stars, 0.0, 1.0);
    
    // Slight blue tint for space feel
    let color = vec3<f32>(0.85, 0.90, 1.0) * stars;
    return vec4<f32>(color, stars);
}
