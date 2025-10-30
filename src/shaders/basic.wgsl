struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) normal: vec3<f32>,
    @location(2) uv: vec2<f32>,
}

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) normal: vec3<f32>,
    @location(1) uv: vec2<f32>,
};

struct PushConstants {
    transform: mat4x4<f32>,
    texture_index: u32,
}

var<push_constant> pc: PushConstants;

@group(0) @binding(0) var textures: binding_array<texture_2d<f32>>;
@group(0) @binding(1) var base_sampler: sampler;

@vertex
fn vs_main(in: VertexInput) -> VertexOutput {
    var out: VertexOutput;
    out.clip_position = pc.transform * vec4<f32>(in.position, 1.0);
    out.normal = in.normal;
    out.uv = in.uv;
    return out;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    let texture_color = textureSample(textures[pc.texture_index], base_sampler, in.uv);

    let lighting = dot(normalize(in.normal), normalize(vec3<f32>(1.0, 1.0, 1.0))) * 0.5 + 0.5;

    return vec4<f32>(texture_color.rgb * lighting, texture_color.a);
}