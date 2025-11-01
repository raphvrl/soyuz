struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) normal: vec3<f32>,
    @location(2) uv: vec2<f32>,
}

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) world_position: vec3<f32>,
    @location(1) normal: vec3<f32>,
    @location(2) uv: vec2<f32>,
}

struct CameraUniform {
    view_proj: mat4x4<f32>,
}

@group(0) @binding(0) var<uniform> camera: CameraUniform;

@group(1) @binding(0) var textures: binding_array<texture_2d<f32>>;
@group(1) @binding(1) var base_sampler: sampler;

struct PointLight {
    position: vec4<f32>,
    color: vec4<f32>,
    intensity: f32,
    radius: f32,
    _padding: vec2<f32>,
}

struct DirectionalLight {
    direction: vec3<f32>,
    _pad0: f32,
    color: vec3<f32>,
    intensity: f32,
}

struct LightingData {
    point_lights: array<PointLight, 16>,
    num_point_lights: u32,
    directional_light: DirectionalLight,
    _padding: vec3<f32>,
}

@group(2) @binding(0) var<uniform> lighting: LightingData;

struct LightSpaceUniform {
    light_space_matrix: mat4x4<f32>,
}

@group(3) @binding(0) var shadow_map: texture_depth_2d;
@group(3) @binding(1) var shadow_sampler: sampler_comparison;
@group(3) @binding(2) var<uniform> light_space: LightSpaceUniform;

struct PushConstants {
    model_matrix: mat4x4<f32>,
    base_color: vec4<f32>,
    texture_index: u32,
}

var<push_constant> pc: PushConstants;

@vertex
fn vs_main(in: VertexInput) -> VertexOutput {
    var out: VertexOutput;

    let world_pos = pc.model_matrix * vec4<f32>(in.position, 1.0);
    out.world_position = world_pos.xyz;

    out.clip_position = camera.view_proj * world_pos;

    out.normal = (pc.model_matrix * vec4<f32>(in.normal, 0.0)).xyz;
    out.uv = in.uv;

    return out;
}

fn calculate_point_light(light: PointLight, world_pos: vec3<f32>, normal: vec3<f32>) -> vec3<f32> {
    let light_position = light.position.xyz;
    let light_color = light.color.xyz;
    let light_dir = light_position - world_pos;
    let distance = length(light_dir);
    let normalized_light_dir = normalize(light_dir);

    let attenuation = max(0.0, 1.0 - (distance / light.radius));
    let attenuation_squared = attenuation * attenuation;

    let diffuse = max(dot(normal, normalized_light_dir), 0.0);

    return light_color * light.intensity * diffuse * attenuation_squared;
}

fn calculate_directional_light(light: DirectionalLight, normal: vec3<f32>) -> vec3<f32> {
    let normalized_light_dir = normalize(-light.direction);
    let diffuse = max(dot(normal, normalized_light_dir), 0.0);
    return light.color * light.intensity * diffuse;
}

fn calculate_shadow(world_pos: vec3<f32>) -> f32 {
    let light_space_pos = light_space.light_space_matrix * vec4<f32>(world_pos, 1.0);

    if light_space_pos.w <= 0.0 {
        return 1.0;
    }

    let flip_correction = vec2<f32>(0.5, -0.5);

    let proj_correction = 1.0 / light_space_pos.w;
    let sample_coords = light_space_pos.xy * flip_correction * proj_correction + vec2<f32>(0.5, 0.5);

    if sample_coords.x < 0.0 || sample_coords.x > 1.0 || sample_coords.y < 0.0 || sample_coords.y > 1.0 {
        return 1.0;
    }

    let shadow = textureSampleCompareLevel(
        shadow_map,
        shadow_sampler,
        sample_coords,
        light_space_pos.z * proj_correction
    );

    return shadow;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    let texture_color = textureSample(textures[pc.texture_index], base_sampler, in.uv);
    let base_color = texture_color * pc.base_color;

    let normal = normalize(in.normal);

    let ambient = vec3<f32>(0.1, 0.1, 0.1);

    var total_light = ambient;
    for (var i = 0u; i < lighting.num_point_lights; i = i + 1u) {
        total_light += calculate_point_light(lighting.point_lights[i], in.world_position, normal);
    }

    let shadow_factor = calculate_shadow(in.world_position);
    let directional_light = calculate_directional_light(lighting.directional_light, normal);
    total_light += directional_light * shadow_factor;

    let final_color = base_color.rgb * total_light;

    return vec4<f32>(final_color, base_color.a);
}