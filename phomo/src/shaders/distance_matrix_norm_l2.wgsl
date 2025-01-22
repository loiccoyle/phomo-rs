struct Dimensions {
    width: u32,
    height: u32,
    channels: u32,
    padding: u32,
}

@group(0) @binding(0) var<storage, read> cell_arrays: array<f32>;
@group(0) @binding(1) var<storage, read> tile_arrays: array<f32>;
@group(0) @binding(2) var<storage, read_write> result: array<f32>;
@group(0) @binding(3) var<uniform> dimensions: Dimensions;


@compute @workgroup_size(256)
fn main(@builtin(global_invocation_id) global_id: vec3<u32>) {
    let idx = global_id.x;
    let master_count = arrayLength(&cell_arrays) / (dimensions.width * dimensions.height * dimensions.channels);
    let pool_count = arrayLength(&tile_arrays) / (dimensions.width * dimensions.height * dimensions.channels);

    if idx >= master_count * pool_count {
        return;
    }

    let i = idx / pool_count;
    let j = idx % pool_count;

    let array_size = dimensions.width * dimensions.height * dimensions.channels;
    let cell_offset = i * array_size;
    let tile_offset = j * array_size;

    var distance: f32 = 0.0;

    for (var k: u32 = 0; k < array_size; k++) {
        let diff = cell_arrays[cell_offset + k] - tile_arrays[tile_offset + k];
        distance += diff * diff;
    }

    result[idx] = sqrt(distance);
}

