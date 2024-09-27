use pollster::block_on;

pub struct Renderer {
    device: wgpu::Device,
    queue: wgpu::Queue,
    surface: wgpu::Surface<'static>,
    surface_config: wgpu::SurfaceConfiguration,
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

        Self {
            device,
            queue,
            surface,
            surface_config,
        }
    }

    pub fn resize(&mut self, new_width: u32, new_height: u32) {
        self.surface_config.width = new_width;
        self.surface_config.height = new_height;
        self.surface.configure(&self.device, &self.surface_config);
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
            let _render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("render_pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color::BLUE),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                ..Default::default()
            });
        }

        self.queue.submit(std::iter::once(encoder.finish()));
        output.present();
    }
}
