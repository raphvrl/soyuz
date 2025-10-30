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
};

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
}

struct LightingData {
    point_lights: array<PointLight, 16>,
    num_point_lights: u32,
}

@group(2) @binding(0) var<uniform> lighting: LightingData;

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

    let final_color = base_color.rgb * total_light;


    return vec4<f32>(final_color, base_color.a);
}