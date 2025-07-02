@group(0) @binding(0) var myTexture: texture_2d<f32>;
@group(0) @binding(1) var mySampler: sampler;

struct Vertex {
    @location(0) position: vec3<f32>,
    @location(1) color: vec3<f32>,
    @location(2) texCoord: vec2<f32>,
}

struct VertexPayload {
    @builtin(position) position: vec4<f32>,
    @location(0) color: vec3<f32>,
    @location(1) texCoord: vec2<f32>,
}

@vertex
fn vs_main(vertex: Vertex) -> VertexPayload {
    var out: VertexPayload;
    out.position = vec4<f32>(vertex.position, 1.0);
    out.color = vertex.color;
    out.texCoord = vertex.texCoord;
    return out;
}

@fragment
fn fs_main(in: VertexPayload) -> @location(0) vec4<f32> {
    var c = textureSample(myTexture, mySampler, in.texCoord).xyz;
    var result = c;
    return vec4<f32>(result, 1.0);
}

// Shading Library
fn filter_scalar(scalar: f32, min: f32, max: f32) -> f32 {
    if scalar < min {
        return min;
    } else if scalar > max {
        return max;
    } else {
        return scalar;
    }
}
fn filter_color(c: vec3<f32>, min: f32, max: f32) -> vec3<f32> {
    return vec3<f32>(
        filter_scalar(c.r, min, max),
        filter_scalar(c.g, min, max),
        filter_scalar(c.b, min, max),
    );
}
fn filter_scalar_and_stretch(scalar: f32, min: f32, max: f32) -> f32 {
    return (filter_scalar(scalar, min, max) - min) * (1.0 / (max - min));
}
fn scalar(scalar: f32) -> vec3<f32> {
    return vec3<f32>(scalar, scalar, scalar);
}
fn get_r(c: vec3<f32>) -> f32 { return c.x; }
fn get_g(c: vec3<f32>) -> f32 { return c.y; }
fn get_b(c: vec3<f32>) -> f32 { return c.z; }
fn mult(a: vec3<f32>, b: vec3<f32>) -> vec3<f32> {
    return vec3<f32>(
        a.x * b.x,
        a.y * b.y,
        a.z * b.z,
    );
}
fn color(r: f32, g: f32, b: f32) -> vec3<f32> {
    return vec3<f32>(r, g, b);
}
fn only_r(r: f32) -> vec3<f32> {
    return vec3<f32>(r, 0.0, 0.0);
}
fn only_g(g: f32) -> vec3<f32> {
    return vec3<f32>(0.0, g, 0.0);
}
fn only_b(b: f32) -> vec3<f32> {
    return vec3<f32>(0.0, 0.0, b);
}
fn add(a: vec3<f32>, b: vec3<f32>) -> vec3<f32> {
    return vec3<f32>(
        a.x + b.x,
        a.y + b.y,
        a.z + b.z,
    );
}
fn sub(a: vec3<f32>, b: vec3<f32>) -> vec3<f32> {
    return add(
        a,
        vec3<f32>(
            -b.r,
            -b.g,
            -b.b,
        ),
    );
}
fn xor_color(a: vec3<f32>, b: vec3<f32>) -> vec3<f32> {
    return color(abs(a.r - b.r), abs(a.g - b.g), abs(a.b - b.b));
}
fn safe_color(c: f32) -> f32 {
    return filter_scalar(c, 0.0, 255.0);
}
