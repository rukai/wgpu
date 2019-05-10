use rayon::prelude::*;

fn main() {
    env_logger::init();

    let width: u16 = 400;
    let height: u16 = 300;

    let foo: Vec<i32> = (0..1000).collect();
    foo.par_iter().for_each(|i| {
        println!("{}", i);
        let mut state = create_state();
        // removing this loop fixes the deadlock
        for j in 0..30 {
            println!("j: {}", j);
            let texture_extent = wgpu::Extent3d {
                width: width as u32,
                height: height as u32,
                depth: 1
            };
            let framebuffer_descriptor = &wgpu::TextureDescriptor {
                size: texture_extent,
                array_size: 1,
                dimension: wgpu::TextureDimension::D2,
                format: wgpu::TextureFormat::Bgra8Unorm,
                usage: wgpu::TextureUsageFlags::all(),
            };
            // commenting out this framebuffer fixes the deadlock
            let framebuffer = state.device.create_texture(framebuffer_descriptor);

            let framebuffer_out_usage = &wgpu::BufferDescriptor {
                size: width as u32 * height as u32 * 4,
                usage: wgpu::BufferUsageFlags::all(),
            };
            let framebuffer_out = state.device.create_buffer(framebuffer_out_usage);

            let command_encoder = state.device.create_command_encoder(&wgpu::CommandEncoderDescriptor { todo: 0 });
            state.device.get_queue().submit(&[command_encoder.finish()]);

            println!("map_read_async");
            framebuffer_out.map_read_async(0, width as u32 * height as u32 * 4, move |result: wgpu::BufferMapAsyncResult<&[u32]>| {
                println!("{:?}", result.unwrap().data.len());
            });
        }
    });
}

struct WgpuState {
    pub instance: wgpu::Instance,
    pub device: wgpu::Device,
    pub vs_module: wgpu::ShaderModule,
    pub fs_module: wgpu::ShaderModule,
    pub bind_group_layout: wgpu::BindGroupLayout,
    pub render_pipeline: wgpu::RenderPipeline,
}

fn create_state() -> WgpuState {
    let instance = wgpu::Instance::new();
    let adapter = instance.get_adapter(&wgpu::AdapterDescriptor {
        power_preference: wgpu::PowerPreference::LowPower,
    });
    let device = adapter.create_device(&wgpu::DeviceDescriptor {
        extensions: wgpu::Extensions {
            anisotropic_filtering: false,
        },
    });

    // shaders
    let vs_bytes = include_bytes!("./../data/hello_triangle.vert.spv");
    let vs_module = device.create_shader_module(vs_bytes);
    let fs_bytes = include_bytes!("./../data/hello_triangle.frag.spv");
    let fs_module = device.create_shader_module(fs_bytes);

    // layout
    let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
        bindings: &[
            wgpu::BindGroupLayoutBinding {
                binding: 0,
                visibility: wgpu::ShaderStageFlags::VERTEX,
                ty: wgpu::BindingType::UniformBuffer,
            },
        ],
    });

    let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
        bind_group_layouts: &[&bind_group_layout],
    });

    let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
        layout: &pipeline_layout,
        vertex_stage: wgpu::PipelineStageDescriptor {
            module: &vs_module,
            entry_point: "main",
        },
        fragment_stage: wgpu::PipelineStageDescriptor {
            module: &fs_module,
            entry_point: "main",
        },
        rasterization_state: wgpu::RasterizationStateDescriptor {
            front_face: wgpu::FrontFace::Ccw,
            cull_mode: wgpu::CullMode::None,
            depth_bias: 0,
            depth_bias_slope_scale: 0.0,
            depth_bias_clamp: 0.0,
        },
        primitive_topology: wgpu::PrimitiveTopology::TriangleList,
        color_states: &[wgpu::ColorStateDescriptor {
            format: wgpu::TextureFormat::Bgra8Unorm,
            color: wgpu::BlendDescriptor::REPLACE,
            alpha: wgpu::BlendDescriptor::REPLACE,
            write_mask: wgpu::ColorWriteFlags::ALL,
        }],
        depth_stencil_state: None,
        index_format: wgpu::IndexFormat::Uint16,
        vertex_buffers: &[],
        sample_count: 1,
    });

    WgpuState {
        instance,
        device,
        vs_module,
        fs_module,
        bind_group_layout,
        render_pipeline,
    }
}
