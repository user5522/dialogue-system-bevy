#import bevy_pbr::forward_io::VertexOutput

@group(2) @binding(0) var<uniform> top_color: vec4<f32>;
@group(2) @binding(1) var<uniform> middle_color: vec4<f32>;
@group(2) @binding(2) var<uniform> bottom_color: vec4<f32>;

@fragment
fn fragment(in: VertexOutput) -> @location(0) vec4<f32> {
    let uv_y = in.uv.y;
    
    var color: vec4<f32>;
    
    if (uv_y > 0.666) {
        color = top_color;
    } else if (uv_y > 0.333) {
        color = middle_color;
    } else {
        color = bottom_color;
    }
    
    return color;
}