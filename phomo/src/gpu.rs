extern crate bytemuck;
extern crate pollster;
extern crate wgpu;
use bytemuck::{Pod, Zeroable};
use util::{BufferInitDescriptor, DeviceExt};
use wgpu::*;

// use crate::error::Error;
use crate::{DistanceMatrix, Mosaic};

pub enum GpuMetricShader {
    NormL1,
    NormL2,
}

impl GpuMetricShader {
    pub fn value(&self) -> String {
        match self {
            GpuMetricShader::NormL1 => {
                include_str!("shaders/distance_matrix_norm_l1.wgsl").to_string()
            }
            GpuMetricShader::NormL2 => {
                include_str!("shaders/distance_matrix_norm_l2.wgsl").to_string()
            }
        }
    }
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Pod, Zeroable)]
struct Dimensions {
    width: u32,
    height: u32,
    channels: u32,
    padding: u32, // for alignment
}

pub struct GpuContext {
    device: Device,
    queue: Queue,
    compute_pipeline: ComputePipeline,
}

impl GpuContext {
    pub async fn new(metric_shader: String) -> Result<Self, Box<dyn std::error::Error>> {
        let instance = Instance::default();
        let adapter = match instance
            .request_adapter(&RequestAdapterOptions {
                power_preference: PowerPreference::HighPerformance,
                force_fallback_adapter: false,
                compatible_surface: None,
            })
            .await
        {
            Some(adapter) => adapter,
            None => {
                return Err("No suitable GPU found".into());
            }
        };

        let (device, queue) = adapter
            .request_device(
                &DeviceDescriptor {
                    label: None,
                    required_features: Features::empty(),
                    required_limits: Limits::default(),
                    memory_hints: Default::default(),
                },
                None,
            )
            .await?;

        // Create shader module
        let shader = device.create_shader_module(ShaderModuleDescriptor {
            label: Some("Distance Matrix Compute Shader"),
            source: ShaderSource::Wgsl(metric_shader.into()),
        });

        // Create compute pipeline
        let compute_pipeline = device.create_compute_pipeline(&ComputePipelineDescriptor {
            label: Some("Distance Matrix Pipeline"),
            layout: None,
            module: &shader,
            entry_point: Some("main"),
            compilation_options: Default::default(),
            cache: Default::default(),
        });

        Ok(Self {
            device,
            queue,
            compute_pipeline,
        })
    }
}

impl Mosaic {
    /// Compute the (flat) distance matrix between the tiles and the master cells using the provided
    /// `metric_shader` [GpuMetricShader] enum member.
    ///
    /// The row index is the cell index and the column index is the tile index.
    pub async fn distance_matrix_gpu_with_metric(
        &self,
        metric_shader: GpuMetricShader,
    ) -> Result<DistanceMatrix<i64>, Box<dyn std::error::Error>> {
        let ctx = GpuContext::new(metric_shader.value()).await?;
        let cell_data: Vec<f32> = self
            .master
            .cells
            .iter()
            .flat_map(|cell| cell.iter().map(|f| *f as f32))
            .collect();
        let tile_data: Vec<f32> = self
            .tiles
            .iter()
            .flat_map(|tile| tile.iter().map(|f| *f as f32))
            .collect();

        let cell_buffer = ctx.device.create_buffer_init(&BufferInitDescriptor {
            label: Some("Cell Arrays Buffer"),
            contents: bytemuck::cast_slice(&cell_data),
            usage: BufferUsages::STORAGE,
        });
        let tile_buffer = ctx.device.create_buffer_init(&BufferInitDescriptor {
            label: Some("Tile Arrays Buffer"),
            contents: bytemuck::cast_slice(&tile_data),
            usage: BufferUsages::STORAGE,
        });

        let dimensions = Dimensions {
            width: self.master.cell_size.0,
            height: self.master.cell_size.1,
            channels: 3,
            padding: 0,
        };
        let dimensions_buffer = ctx.device.create_buffer_init(&BufferInitDescriptor {
            label: Some("Dimensions Buffer"),
            contents: bytemuck::bytes_of(&dimensions),
            usage: BufferUsages::UNIFORM,
        });

        let result_size = (self.master.cells.len() * self.tiles.len()) as u64;
        let result_buffer = ctx.device.create_buffer(&BufferDescriptor {
            label: Some("Result Buffer"),
            size: result_size * std::mem::size_of::<f32>() as u64,
            usage: BufferUsages::STORAGE | BufferUsages::COPY_SRC,
            mapped_at_creation: false,
        });

        let bind_group_layout = ctx.compute_pipeline.get_bind_group_layout(0);
        let bind_group = ctx.device.create_bind_group(&BindGroupDescriptor {
            label: Some("Distance Matrix Bind Group"),
            layout: &bind_group_layout,
            entries: &[
                BindGroupEntry {
                    binding: 0,
                    resource: cell_buffer.as_entire_binding(),
                },
                BindGroupEntry {
                    binding: 1,
                    resource: tile_buffer.as_entire_binding(),
                },
                BindGroupEntry {
                    binding: 2,
                    resource: result_buffer.as_entire_binding(),
                },
                BindGroupEntry {
                    binding: 3,
                    resource: dimensions_buffer.as_entire_binding(),
                },
            ],
        });

        let mut encoder = ctx
            .device
            .create_command_encoder(&CommandEncoderDescriptor {
                label: Some("Distance Matrix Encoder"),
            });

        {
            let mut compute_pass = encoder.begin_compute_pass(&ComputePassDescriptor {
                label: Some("Distance Matrix Compute Pass"),
                timestamp_writes: Default::default(),
            });
            compute_pass.set_pipeline(&ctx.compute_pipeline);
            compute_pass.set_bind_group(0, &bind_group, &[]);

            let workgroup_size = 256;
            let num_workgroups = ((self.master.cells.len() * self.tiles.len()) as f32
                / workgroup_size as f32)
                .ceil() as u32;
            compute_pass.dispatch_workgroups(num_workgroups, 1, 1);
        }

        let staging_buffer = ctx.device.create_buffer(&BufferDescriptor {
            label: Some("Staging Buffer"),
            size: result_size * std::mem::size_of::<f32>() as u64,
            usage: BufferUsages::MAP_READ | BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });
        encoder.copy_buffer_to_buffer(&result_buffer, 0, &staging_buffer, 0, staging_buffer.size());
        ctx.queue.submit(Some(encoder.finish()));

        let slice = staging_buffer.slice(..);
        slice.map_async(MapMode::Read, move |_| {});
        ctx.device.poll(Maintain::Wait);

        let data = slice.get_mapped_range();
        let result: Vec<i64> = bytemuck::cast_slice(&data)
            .iter()
            .map(|&x: &f32| x as i64)
            .collect();
        drop(data);
        staging_buffer.unmap();

        Ok(DistanceMatrix {
            rows: self.master.cells.len(),
            columns: self.tiles.len(),
            data: result,
        })
    }

    /// Compute the (flat) distance matrix between the tiles and the master cells, using the
    /// [`GpuMetricShader::NormL1`] metric.
    ///
    /// To use a different distance metric, use the [`distance_matrix_gpu_with_metric`](Mosaic::distance_matrix_gpu_with_metric) method.
    ///
    /// The row index is the cell index and the column index is the tile index.
    pub async fn distance_matrix_gpu(
        &self,
    ) -> Result<DistanceMatrix<i64>, Box<dyn std::error::Error>> {
        self.distance_matrix_gpu_with_metric(GpuMetricShader::NormL1)
            .await
    }

    /// Blocking variant of the [`distance_matrix_gpu_with_metric`](Mosaic::distance_matrix_gpu_with_metric) function.
    pub fn distance_matrix_gpu_with_metric_blocking(
        &self,
        metric_shader: GpuMetricShader,
    ) -> Result<DistanceMatrix<i64>, Box<dyn std::error::Error>> {
        pollster::block_on(self.distance_matrix_gpu_with_metric(metric_shader))
    }

    /// Blocking variant of the [`distance_matrix_gpu`](Mosaic::distance_matrix_gpu) function.
    pub fn distance_matrix_gpu_blocking(
        &self,
    ) -> Result<DistanceMatrix<i64>, Box<dyn std::error::Error>> {
        pollster::block_on(self.distance_matrix_gpu())
    }
}
