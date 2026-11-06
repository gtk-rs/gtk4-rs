use std::{
    cell::{Cell, RefCell},
    num::NonZeroU32,
    time::Instant,
};

use gtk::{glib, prelude::*, subclass::prelude::*};
use wgpu::hal::{api::Gles, gles};

/// The wgpu objects, created once the `GLArea` has a GL context.
struct Renderer {
    device: wgpu::Device,
    queue: wgpu::Queue,
    pipeline: wgpu::RenderPipeline,
    uniform_buffer: wgpu::Buffer,
    bind_group: wgpu::BindGroup,
}

const FORMAT: wgpu::TextureFormat = wgpu::TextureFormat::Rgba8Unorm;

impl Renderer {
    fn new(area: &gtk::GLArea) -> Self {
        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
            backends: wgpu::Backends::GL,
            ..wgpu::InstanceDescriptor::new_without_display_handle()
        });
        // Creating the instance can change the current GL context (on Windows, wgpu
        // probes WGL with a context of its own), so make the context of the GLArea
        // current again before handing it to wgpu.
        area.make_current();

        // SAFETY: the GL context of the `GLArea` is current.
        let exposed = unsafe {
            gles::Adapter::new_external(crate::get_proc_address, wgpu::GlBackendOptions::default())
        }
        .expect("Failed to create a wgpu adapter from the GTK GL context");

        // SAFETY: the adapter was just created from the current GL context.
        let adapter = unsafe { instance.create_adapter_from_hal::<Gles>(exposed) };
        let (device, queue) = glib::MainContext::default()
            .block_on(adapter.request_device(&wgpu::DeviceDescriptor {
                required_limits:
                    wgpu::Limits::downlevel_webgl2_defaults().using_resolution(adapter.limits()),
                ..Default::default()
            }))
            .expect("Failed to create a wgpu device");

        let shader = device.create_shader_module(wgpu::include_wgsl!("shader.wgsl"));
        let uniform_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("angle"),
            // Uniform buffers must be at least 16 bytes on WebGL2/GLES.
            size: 16,
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
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
                resource: uniform_buffer.as_entire_binding(),
            }],
        });
        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: None,
            bind_group_layouts: &[Some(&bind_group_layout)],
            ..Default::default()
        });
        let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: None,
            layout: Some(&pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: Some("vs_main"),
                buffers: &[],
                compilation_options: Default::default(),
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: Some("fs_main"),
                targets: &[Some(FORMAT.into())],
                compilation_options: Default::default(),
            }),
            primitive: Default::default(),
            depth_stencil: None,
            multisample: Default::default(),
            multiview_mask: None,
            cache: None,
        });

        Self {
            device,
            queue,
            pipeline,
            uniform_buffer,
            bind_group,
        }
    }

    fn render(&self, framebuffer: NonZeroU32, width: u32, height: u32, angle: f32) {
        // Wrap the framebuffer that GTK wants us to draw into as a wgpu texture.
        let mut hal_texture = gles::Texture::default_framebuffer(FORMAT);
        hal_texture.inner = gles::TextureInner::ExternalNativeFramebuffer {
            inner: glow_wgpu::NativeFramebuffer(framebuffer),
        };
        // SAFETY: the texture describes the framebuffer currently bound by GTK, which
        // stays valid for the duration of this `render` call.
        let texture = unsafe {
            self.device.create_texture_from_hal::<Gles>(
                hal_texture,
                &wgpu::TextureDescriptor {
                    label: Some("GtkGLArea framebuffer"),
                    size: wgpu::Extent3d {
                        width,
                        height,
                        depth_or_array_layers: 1,
                    },
                    mip_level_count: 1,
                    sample_count: 1,
                    dimension: wgpu::TextureDimension::D2,
                    format: FORMAT,
                    usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
                    view_formats: &[],
                },
                wgpu::TextureUses::UNINITIALIZED,
            )
        };
        let view = texture.create_view(&Default::default());

        let mut uniform = [0u8; 16];
        uniform[..4].copy_from_slice(&angle.to_ne_bytes());
        self.queue.write_buffer(&self.uniform_buffer, 0, &uniform);

        let mut encoder = self.device.create_command_encoder(&Default::default());
        {
            let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: None,
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    depth_slice: None,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: 0.1,
                            g: 0.1,
                            b: 0.1,
                            a: 1.0,
                        }),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                ..Default::default()
            });
            pass.set_pipeline(&self.pipeline);
            pass.set_bind_group(0, &self.bind_group, &[]);
            pass.draw(0..3, 0..1);
        }
        self.queue.submit([encoder.finish()]);
    }
}

pub struct WgpuArea {
    renderer: RefCell<Option<Renderer>>,
    start_time: Cell<Instant>,
}

impl Default for WgpuArea {
    fn default() -> Self {
        Self {
            renderer: Default::default(),
            start_time: Cell::new(Instant::now()),
        }
    }
}

#[glib::object_subclass]
impl ObjectSubclass for WgpuArea {
    const NAME: &'static str = "WgpuArea";
    type Type = super::WgpuArea;
    type ParentType = gtk::GLArea;
}

impl ObjectImpl for WgpuArea {
    fn constructed(&self) {
        self.parent_constructed();
        self.obj().add_tick_callback(|area, _| {
            area.queue_render();
            glib::ControlFlow::Continue
        });
    }
}

impl WidgetImpl for WgpuArea {
    fn realize(&self) {
        self.parent_realize();
        self.start_time.set(Instant::now());
    }

    fn unrealize(&self) {
        // The wgpu objects own GL resources, so drop them while the context is current.
        self.obj().make_current();
        self.renderer.replace(None);
        self.parent_unrealize();
    }
}

impl GLAreaImpl for WgpuArea {
    fn render(&self, _context: &gtk::gdk::GLContext) -> glib::Propagation {
        use glow_wgpu::HasContext;

        if self.obj().error().is_some() {
            return glib::Propagation::Stop;
        }
        self.ensure_renderer();

        // SAFETY: GTK has made the GL context current and bound the framebuffer
        // it wants us to draw into before emitting `render`.
        let framebuffer = unsafe {
            let gl = glow_wgpu::Context::from_loader_function(crate::get_proc_address);
            gl.get_parameter_i32(glow_wgpu::DRAW_FRAMEBUFFER_BINDING) as u32
        };
        let Some(framebuffer) = NonZeroU32::new(framebuffer) else {
            return glib::Propagation::Stop;
        };

        let area = self.obj();
        let scale_factor = area.scale_factor();
        let width = (area.width() * scale_factor) as u32;
        let height = (area.height() * scale_factor) as u32;
        let angle = self.start_time.get().elapsed().as_secs_f32();

        if let Some(renderer) = self.renderer.borrow().as_ref() {
            renderer.render(framebuffer, width, height, angle);
        }

        glib::Propagation::Stop
    }
}

impl WgpuArea {
    fn ensure_renderer(&self) {
        if self.renderer.borrow().is_none() {
            self.renderer
                .replace(Some(Renderer::new(self.obj().upcast_ref())));
        }
    }
}
