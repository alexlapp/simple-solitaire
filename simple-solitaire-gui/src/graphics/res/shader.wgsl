// Vertex Shader

struct VertexInput {
    @location(0) position: vec3<f32>,
}

struct InstanceInput {
    @location(5) src_rect: vec4<f32>,
    @location(6) pos_matrix_0: vec4<f32>,
    @location(7) pos_matrix_1: vec4<f32>,
    @location(8) pos_matrix_2: vec4<f32>,
    @location(9) pos_matrix_3: vec4<f32>,
    @location(10) size_matrix_0: vec4<f32>,
    @location(11) size_matrix_1: vec4<f32>,
    @location(12) size_matrix_2: vec4<f32>,
    @location(13) size_matrix_3: vec4<f32>,
}

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) tex_coords: vec2<f32>,
};

struct DrawUniforms {
    proj_matrix: mat4x4<f32>,
    scale_matrix: mat4x4<f32>,
}

@group(1) @binding(0)
var<uniform> uniforms: DrawUniforms;


@vertex
fn vs_main(
    model: VertexInput,
    instance: InstanceInput,
) -> VertexOutput {
    let pos_matrix = mat4x4<f32>(
        instance.pos_matrix_0,
        instance.pos_matrix_1,
        instance.pos_matrix_2,
        instance.pos_matrix_3,
    );
    let size_matrix = mat4x4<f32>(
        instance.size_matrix_0,
        instance.size_matrix_1,
        instance.size_matrix_2,
        instance.size_matrix_3,
    );

    var out: VertexOutput;
    out.tex_coords = mix(instance.src_rect.xy, instance.src_rect.zw, model.position.xy);
    // out.clip_position = vec4<f32>(model.position, 1.0);
    // out.clip_position = uniforms.transform * vec4<f32>(model.position.xy, 0.0, 1.0);
    // out.clip_position = vec4<f32>(model.position.xy, 0.0, 1.0);

    out.clip_position = uniforms.proj_matrix * pos_matrix * uniforms.scale_matrix * size_matrix * vec4<f32>(model.position.xy, 0.0, 1.0);

    return out;
}

// Fragment Shader

@group(0) @binding(0)
var t_diffuse: texture_2d<f32>;
@group(0) @binding(1)
var s_diffuse: sampler;

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    // return vec4<f32>(0.3, 0.3, 0.3, 1.0);
    return textureSample(t_diffuse, s_diffuse, in.tex_coords);
}
