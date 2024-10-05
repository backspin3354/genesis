use pollster::block_on;

mod camera;
pub use camera::Camera;
use wgpu::util::DeviceExt;

#[repr(C)]
#[derive(Clone, Copy, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Vertex {
    pub position: glam::Vec3,
    pub color: glam::Vec3,
}

impl Vertex {
    const LAYOUT: wgpu::VertexBufferLayout<'static> = wgpu::VertexBufferLayout {
        array_stride: size_of::<Self>() as u64,
        step_mode: wgpu::VertexStepMode::Vertex,
        attributes: &wgpu::vertex_attr_array![0 => Float32x3, 1 => Float32x3],
    };
}

#[repr(C)]
#[derive(Clone, Copy, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Billboard {
    pub position: glam::Vec3,
    pub color: glam::Vec3,
}

impl Billboard {
    const LAYOUT: wgpu::VertexBufferLayout<'static> = wgpu::VertexBufferLayout {
        array_stride: size_of::<Self>() as u64,
        step_mode: wgpu::VertexStepMode::Instance,
        attributes: &wgpu::vertex_attr_array![2 => Float32x3, 3 => Float32x3],
    };
}

pub struct Renderer {
    device: wgpu::Device,
    queue: wgpu::Queue,
    surface: wgpu::Surface<'static>,
    surface_config: wgpu::SurfaceConfiguration,

    camera_buffer: wgpu::Buffer,
    camera_bind_group: wgpu::BindGroup,

    pipeline: wgpu::RenderPipeline,
    vertex_buffer: wgpu::Buffer,
    index_buffer: wgpu::Buffer,
    num_indices: u32,

    billboard_pipeline: wgpu::RenderPipeline,
    billboard_vertex_buffer: wgpu::Buffer,
    billboard_index_buffer: wgpu::Buffer,
    billboard_instance_buffer: wgpu::Buffer,
    billboard_num_instances: u32,
}

impl Renderer {
    pub fn new<S>(surface: S) -> Self
    where
        S: Into<wgpu::SurfaceTarget<'static>>,
    {
        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
            backends: wgpu::Backends::all(),
            flags: wgpu::InstanceFlags::empty(),
            ..Default::default()
        });

        let surface = instance.create_surface(surface).unwrap();

        let adapter = block_on(instance.request_adapter(&wgpu::RequestAdapterOptions {
            compatible_surface: Some(&surface),
            ..Default::default()
        }))
        .unwrap();

        log::debug!("Selected {:#?}", adapter.get_info());

        let surface_config = wgpu::SurfaceConfiguration {
            present_mode: wgpu::PresentMode::Fifo,
            ..surface.get_default_config(&adapter, 0, 0).unwrap()
        };

        log::debug!("Selected {:#?}", surface_config);

        let (device, queue) = block_on(adapter.request_device(
            &wgpu::DeviceDescriptor {
                required_features: wgpu::Features::empty(),
                required_limits: wgpu::Limits::downlevel_webgl2_defaults(),
                ..Default::default()
            },
            None,
        ))
        .unwrap();

        let camera_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("camera_buffer"),
            size: size_of::<glam::Mat4>() as u64 * 2,
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        let camera_bind_group_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                label: Some("camera_bind_group_layout"),
                entries: &[wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::VERTEX,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                }],
            });

        let camera_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("camera_bind_group"),
            layout: &camera_bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: camera_buffer.as_entire_binding(),
            }],
        });

        let shader = device.create_shader_module(wgpu::include_wgsl!("shader.wgsl"));

        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("pipeline_layout"),
            bind_group_layouts: &[&camera_bind_group_layout],
            push_constant_ranges: &[],
        });

        let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("pipeline"),
            layout: Some(&pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: "vs_main",
                compilation_options: Default::default(),
                buffers: &[Vertex::LAYOUT],
            },
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Cw,
                cull_mode: Some(wgpu::Face::Back),
                unclipped_depth: false,
                polygon_mode: wgpu::PolygonMode::Fill,
                conservative: false,
            },
            depth_stencil: None,
            multisample: wgpu::MultisampleState {
                count: 1,
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: "fs_main",
                compilation_options: Default::default(),
                targets: &[Some(wgpu::ColorTargetState {
                    format: surface_config.format,
                    blend: Some(wgpu::BlendState::REPLACE),
                    write_mask: wgpu::ColorWrites::all(),
                })],
            }),
            multiview: None,
            cache: None,
        });

        let vertex_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("vertex_buffer"),
            size: size_of::<Vertex>() as u64 * 4 * 128,
            usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        let index_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("index_buffer"),
            size: size_of::<u16>() as u64 * 6 * 128,
            usage: wgpu::BufferUsages::INDEX | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        let billboard_shader = device.create_shader_module(wgpu::include_wgsl!("billboard.wgsl"));

        let billboard_pipeline_layout =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("billboard_pipeline_layout"),
                bind_group_layouts: &[&camera_bind_group_layout],
                push_constant_ranges: &[],
            });

        let billboard_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("pipeline"),
            layout: Some(&billboard_pipeline_layout),
            vertex: wgpu::VertexState {
                module: &billboard_shader,
                entry_point: "vs_main",
                compilation_options: Default::default(),
                buffers: &[Vertex::LAYOUT, Billboard::LAYOUT],
            },
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Cw,
                cull_mode: None,
                unclipped_depth: false,
                polygon_mode: wgpu::PolygonMode::Fill,
                conservative: false,
            },
            depth_stencil: None,
            multisample: wgpu::MultisampleState {
                count: 1,
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: "fs_main",
                compilation_options: Default::default(),
                targets: &[Some(wgpu::ColorTargetState {
                    format: surface_config.format,
                    blend: Some(wgpu::BlendState::REPLACE),
                    write_mask: wgpu::ColorWrites::all(),
                })],
            }),
            multiview: None,
            cache: None,
        });

        let billboard_vertex_buffer =
            device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("billboard_vertex_buffer"),
                contents: bytemuck::cast_slice(&[
                    Vertex {
                        position: glam::Vec3::new(-0.5, -0.5, 0.0),
                        color: glam::Vec3::splat(1.0),
                    },
                    Vertex {
                        position: glam::Vec3::new(-0.5, 0.5, 0.0),
                        color: glam::Vec3::splat(0.1),
                    },
                    Vertex {
                        position: glam::Vec3::new(0.5, 0.5, 0.0),
                        color: glam::Vec3::splat(1.0),
                    },
                    Vertex {
                        position: glam::Vec3::new(0.5, -0.5, 0.0),
                        color: glam::Vec3::splat(0.1),
                    },
                ]),
                usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
            });

        let billboard_index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("billboard_index_buffer"),
            contents: bytemuck::cast_slice(&[0u16, 1u16, 2u16, 2u16, 3u16, 0u16]),
            usage: wgpu::BufferUsages::INDEX | wgpu::BufferUsages::COPY_DST,
        });

        let billboard_instance_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("billboard_instance_buffer"),
            size: size_of::<Billboard>() as u64 * 64,
            usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        Self {
            device,
            queue,
            surface,
            surface_config,

            camera_buffer,
            camera_bind_group,

            pipeline,
            vertex_buffer,
            index_buffer,
            num_indices: 0,

            billboard_pipeline,
            billboard_vertex_buffer,
            billboard_index_buffer,
            billboard_instance_buffer,
            billboard_num_instances: 0,
        }
    }

    pub fn resize(&mut self, new_width: u32, new_height: u32) {
        self.surface_config.width = new_width;
        self.surface_config.height = new_height;
        self.surface.configure(&self.device, &self.surface_config);
    }

    pub fn load_mesh(&mut self, vertices: &[Vertex], indices: &[u16]) {
        self.queue
            .write_buffer(&self.vertex_buffer, 0, bytemuck::cast_slice(vertices));
        self.queue
            .write_buffer(&self.index_buffer, 0, bytemuck::cast_slice(indices));
        self.num_indices = indices.len().try_into().unwrap();
    }

    pub fn load_billboards(&mut self, billboards: &[Billboard]) {
        self.queue.write_buffer(
            &self.billboard_instance_buffer,
            0,
            bytemuck::cast_slice(billboards),
        );
        self.billboard_num_instances = billboards.len().try_into().unwrap();
    }

    pub fn load_camera(&mut self, camera: &camera::Camera) {
        self.queue.write_buffer(
            &self.camera_buffer,
            0,
            bytemuck::cast_slice(&[
                camera.view(),
                camera.projection(
                    self.surface_config.width as f32,
                    self.surface_config.height as f32,
                ),
            ]),
        );
    }

    pub fn draw(&mut self) {
        let output = self.surface.get_current_texture().unwrap();
        let view = output
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor::default());

        {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("render_pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color::BLACK),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                ..Default::default()
            });

            render_pass.set_pipeline(&self.pipeline);
            render_pass.set_bind_group(0, &self.camera_bind_group, &[]);
            render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
            render_pass.set_index_buffer(self.index_buffer.slice(..), wgpu::IndexFormat::Uint16);
            render_pass.draw_indexed(0..self.num_indices, 0, 0..1);

            render_pass.set_pipeline(&self.billboard_pipeline);
            render_pass.set_bind_group(0, &self.camera_bind_group, &[]);
            render_pass.set_vertex_buffer(0, self.billboard_vertex_buffer.slice(..));
            render_pass.set_vertex_buffer(1, self.billboard_instance_buffer.slice(..));
            render_pass.set_index_buffer(
                self.billboard_index_buffer.slice(..),
                wgpu::IndexFormat::Uint16,
            );
            render_pass.draw_indexed(0..6, 0, 0..self.billboard_num_instances);
        }

        self.queue.submit(std::iter::once(encoder.finish()));
        output.present();
    }
}
