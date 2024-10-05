struct CameraUniform {
    view: mat4x4<f32>,
    projection: mat4x4<f32>,
};

@group(0) @binding(0)
var<uniform> camera: CameraUniform;

struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) color: vec3<f32>,
};

struct InstanceInput {
    @location(2) position: vec3<f32>,
    @location(3) color: vec3<f32>,  
};

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) color: vec3<f32>,
};

@vertex
fn vs_main(
    vertex: VertexInput,
    instance: InstanceInput,
) -> VertexOutput {
    let translation = mat4x4<f32>(
        1.0, 0.0, 0.0, 0.0,
        0.0, 1.0, 0.0, 0.0,  
        0.0, 0.0, 1.0, 0.0,  
        instance.position.x, instance.position.y, instance.position.z, 1.0,
    );

    var model_view = camera.view * translation;
    
    model_view[0][0] = 1.0;
    model_view[0][1] = 0.0;
    model_view[0][2] = 0.0;
    
    model_view[2][0] = 0.0;
    model_view[2][1] = 0.0;
    model_view[2][2] = 1.0;

    var out: VertexOutput;
    out.clip_position = camera.projection * model_view * vec4<f32>(vertex.position, 1.0);
    out.color = vertex.color * instance.color;
    return out;
};

@fragment
fn fs_main(
    in: VertexOutput
) -> @location(0) vec4<f32> {
    return vec4<f32>(in.color, 1.0);
}
