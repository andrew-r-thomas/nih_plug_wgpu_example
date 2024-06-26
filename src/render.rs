use std::sync::Arc;

use baseview::MouseEvent;
use baseview::Size;
use baseview::Window;
use baseview::WindowHandle;
use baseview::WindowHandler;
use baseview::WindowOpenOptions;
use nih_plug::context::gui::GuiContext;
use nih_plug::editor::ParentWindowHandle;
use nih_plug::params::Param;
use wgpu::{
    util::DeviceExt, BindGroup, Buffer, Device, Queue, RenderPipeline, Surface, SurfaceCapabilities,
};

use crate::NihPlugWgpuExampleParams;

const WINDOW_SIZE: u32 = 512;

pub struct WgpuRenderer {
    pipeline: RenderPipeline,
    device: Device,
    surface: Surface,
    queue: Queue,
    vertex_buffer: Buffer,
    mouse_pos_buffer: Buffer,
    surface_caps: SurfaceCapabilities,
    bind_group: BindGroup,
    size: (u32, u32),
    context: Arc<dyn GuiContext>,
    params: Arc<NihPlugWgpuExampleParams>,
}

impl WgpuRenderer {
    pub async fn new(
        window: &mut Window<'_>,
        context: Arc<dyn GuiContext>,
        params: Arc<NihPlugWgpuExampleParams>,
    ) -> Self {
        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor::default());
        let surface = unsafe { instance.create_surface(&*window) }.unwrap();
        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::default(),
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            })
            .await
            .unwrap();
        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    features: wgpu::Features::empty(),
                    limits: wgpu::Limits::default(),
                    label: None,
                },
                None,
            )
            .await
            .unwrap();

        let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Vertex Buffer"),
            contents: bytemuck::cast_slice(VERTICES_START),
            usage: wgpu::BufferUsages::VERTEX,
        });

        let mouse_pos_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("mouse position buffer"),
            contents: bytemuck::cast_slice(&[0.0, 0.0]),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });
        let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: None,
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
        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: None,
            layout: &bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: mouse_pos_buffer.as_entire_binding(),
            }],
        });

        let surface_caps = surface.get_capabilities(&adapter);
        let surface_format = surface_caps
            .formats
            .iter()
            .copied()
            .find(|f| f.is_srgb())
            .unwrap_or(surface_caps.formats[0]);
        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width: WINDOW_SIZE,
            height: WINDOW_SIZE,
            present_mode: surface_caps.present_modes[0],
            alpha_mode: surface_caps.alpha_modes[0],
            view_formats: vec![],
        };

        let shader = device.create_shader_module(wgpu::include_wgsl!("shader.wgsl"));

        let render_pipeline_layout =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("Render Pipeline Layout"),
                bind_group_layouts: &[&bind_group_layout],
                push_constant_ranges: &[],
            });

        let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Render Pipeline"),
            layout: Some(&render_pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: "vs_main",
                buffers: &[Vertex::desc()],
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: "fs_main",
                targets: &[Some(wgpu::ColorTargetState {
                    format: config.format,
                    blend: Some(wgpu::BlendState::REPLACE),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: Some(wgpu::Face::Back),
                polygon_mode: wgpu::PolygonMode::Fill,
                unclipped_depth: false,
                conservative: false,
            },
            depth_stencil: None,
            multisample: wgpu::MultisampleState {
                count: 1,
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
            multiview: None,
        });

        surface.configure(&device, &config);

        Self {
            pipeline,
            device,
            surface,
            queue,
            vertex_buffer,
            surface_caps,
            bind_group,
            mouse_pos_buffer,
            size: (WINDOW_SIZE, WINDOW_SIZE),
            context,
            params,
        }
    }

    pub fn start(
        parent: ParentWindowHandle,
        context: Arc<dyn GuiContext>,
        params: Arc<NihPlugWgpuExampleParams>,
    ) -> WgpuWindowHandle {
        let window_open_options = WindowOpenOptions {
            title: "wgpu on baseview".into(),
            size: Size::new(WINDOW_SIZE as f64, WINDOW_SIZE as f64),
            scale: baseview::WindowScalePolicy::SystemScaleFactor,
        };

        let bv_handle =
            Window::open_parented(&parent, window_open_options, move |window: &mut Window| {
                pollster::block_on(WgpuRenderer::new(window, context, params))
            });

        WgpuWindowHandle { bv_handle }
    }
}

pub struct WgpuWindowHandle {
    bv_handle: WindowHandle,
}
unsafe impl Send for WgpuWindowHandle {}

impl WindowHandler for WgpuRenderer {
    fn on_frame(&mut self, _window: &mut baseview::Window) {
        let output = self.surface.get_current_texture().unwrap();
        let view = output
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Render Encoder"),
            });

        {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Render Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: 0.0,
                            g: 0.0,
                            b: 0.0,
                            a: 1.0,
                        }),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: None,
                occlusion_query_set: None,
                timestamp_writes: None,
            });

            render_pass.set_pipeline(&self.pipeline);
            render_pass.set_bind_group(0, &self.bind_group, &[]);
            render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
            render_pass.draw(0..3, 0..1);
        }

        self.queue.submit(std::iter::once(encoder.finish()));
        output.present();
    }
    fn on_event(
        &mut self,
        _window: &mut baseview::Window,
        event: baseview::Event,
    ) -> baseview::EventStatus {
        match event {
            baseview::Event::Mouse(MouseEvent::CursorMoved {
                position,
                modifiers: _,
            }) => {
                let ptr = self.params.gain.as_ptr();
                unsafe {
                    self.context.raw_begin_set_parameter(ptr);
                }
                let center_x: f32 =
                    (position.x as f32 - (self.size.0 as f32 / 2.0)) / (self.size.0 as f32 / 2.0);
                let center_y: f32 =
                    ((self.size.1 as f32 / 2.0) - position.y as f32) / (self.size.1 as f32 / 2.0);

                let dist = f32::sqrt((center_x * center_x) + (center_y * center_y));

                let gain = self.params.gain.preview_normalized(1.0 - dist);
                unsafe {
                    self.context.raw_set_parameter_normalized(ptr, gain);
                }

                self.queue.write_buffer(
                    &self.mouse_pos_buffer,
                    0,
                    bytemuck::cast_slice(&[center_x, center_y]),
                );
                unsafe {
                    self.context.raw_end_set_parameter(ptr);
                }
            }
            baseview::Event::Window(baseview::WindowEvent::Resized(size)) => {
                let width = size.physical_size().width;
                let height = size.physical_size().height;

                let surface_format = self
                    .surface_caps
                    .formats
                    .iter()
                    .copied()
                    .find(|f| f.is_srgb())
                    .unwrap_or(self.surface_caps.formats[0]);

                self.surface.configure(
                    &self.device,
                    &wgpu::SurfaceConfiguration {
                        usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
                        format: surface_format,
                        width,
                        height,
                        present_mode: self.surface_caps.present_modes[0],
                        alpha_mode: self.surface_caps.alpha_modes[0],
                        view_formats: vec![],
                    },
                );
            }
            _ => {}
        }
        baseview::EventStatus::Captured
    }
}

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
struct Vertex {
    position: [f32; 3],
}

const VERTICES_START: &[Vertex] = &[
    Vertex {
        position: [0.0, 0.25, 0.0],
    },
    Vertex {
        position: [-0.25, -0.25, 0.0],
    },
    Vertex {
        position: [0.25, -0.25, 0.0],
    },
];

impl Vertex {
    const ATTRIBS: [wgpu::VertexAttribute; 2] =
        wgpu::vertex_attr_array![0 => Float32x3, 1 => Float32x3];

    fn desc() -> wgpu::VertexBufferLayout<'static> {
        use std::mem;

        wgpu::VertexBufferLayout {
            array_stride: mem::size_of::<Self>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &Self::ATTRIBS,
        }
    }
}
