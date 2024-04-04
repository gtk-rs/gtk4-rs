use std::{cell::RefCell, rc::Rc};

use glium::{
    implement_vertex, index::PrimitiveType, program, uniform, Frame, IndexBuffer, Surface,
    VertexBuffer,
};
use gtk::{glib, prelude::*, subclass::prelude::*};

#[derive(Copy, Clone)]
struct Vertex {
    position: [f32; 2],
    color: [f32; 3],
}

implement_vertex!(Vertex, position, color);

struct Renderer {
    context: Rc<glium::backend::Context>,
    vertex_buffer: VertexBuffer<Vertex>,
    index_buffer: IndexBuffer<u16>,
    program: glium::Program,
}

impl Renderer {
    fn new(context: Rc<glium::backend::Context>) -> Self {
        // The following code is based on glium's triangle example:
        // https://github.com/glium/glium/blob/2ff5a35f6b097889c154b42ad0233c6cdc6942f4/examples/triangle.rs
        let vertex_buffer = VertexBuffer::new(
            &context,
            &[
                Vertex {
                    position: [-0.5, -0.5],
                    color: [0., 1., 0.],
                },
                Vertex {
                    position: [0., 0.5],
                    color: [0., 0., 1.],
                },
                Vertex {
                    position: [0.5, -0.5],
                    color: [1., 0., 0.],
                },
            ],
        )
        .unwrap();
        let index_buffer =
            IndexBuffer::new(&context, PrimitiveType::TrianglesList, &[0u16, 1, 2]).unwrap();
        let program = program!(&context,
            // This example includes a shader that requires GLSL 140 or above.
            //
            // Emmanuele Bassi explains:
            //
            // The version of the shaders depend on the version of GL; currently, GTK4 uses:
            //
            // - 100 on GLES
            // - 110 on GL2 legacy
            // - 130 on GL3 legacy
            // - 150 on GL3.2+
            //
            // In practice, the version of GLSL for the shaders inside your application depends on
            // the GL context you're either creating or using—i.e. if you support multiple versions
            // of GL then you should load different shaders.
            //
            // If you only care about recent GL, as you should, then going for GLSL 1.50 is
            // perfectly fine; anything else will error out, and you can catch that error and fall
            // back to something else.
            140 => {
                vertex: "
                    #version 140
                    uniform mat4 matrix;
                    in vec2 position;
                    in vec3 color;
                    out vec3 vColor;
                    void main() {
                        gl_Position = vec4(position, 0.0, 1.0) * matrix;
                        vColor = color;
                    }
                ",

                fragment: "
                    #version 140
                    in vec3 vColor;
                    out vec4 f_color;
                    void main() {
                        f_color = vec4(vColor, 1.0);
                    }
                "
            },
        )
        .unwrap();

        Renderer {
            context,
            vertex_buffer,
            index_buffer,
            program,
        }
    }

    fn draw(&self) {
        let mut frame = Frame::new(
            self.context.clone(),
            self.context.get_framebuffer_dimensions(),
        );

        let uniforms = uniform! {
            matrix: [
                [1., 0., 0., 0.],
                [0., 1., 0., 0.],
                [0., 0., 1., 0.],
                [0., 0., 0., 1f32]
            ]
        };

        frame.clear_color(0., 0., 0., 0.);
        frame
            .draw(
                &self.vertex_buffer,
                &self.index_buffer,
                &self.program,
                &uniforms,
                &Default::default(),
            )
            .unwrap();
        frame.finish().unwrap();
    }
}

#[derive(Default)]
pub struct GliumGLArea {
    renderer: RefCell<Option<Renderer>>,
}

#[glib::object_subclass]
impl ObjectSubclass for GliumGLArea {
    const NAME: &'static str = "GliumGLArea";
    type Type = super::GliumGLArea;
    type ParentType = gtk::GLArea;
}

impl ObjectImpl for GliumGLArea {}

impl WidgetImpl for GliumGLArea {
    fn realize(&self) {
        self.parent_realize();

        let widget = self.obj();
        if widget.error().is_some() {
            return;
        }

        // SAFETY: we know the GdkGLContext exists as we checked for errors above, and
        // we haven't done any operations on it which could lead to glium's
        // state mismatch. (In theory, GTK doesn't do any state-breaking
        // operations on the context either.)
        //
        // We will also ensure glium's context does not outlive the GdkGLContext by
        // destroying it in `unrealize()`.
        let context =
            unsafe { glium::backend::Context::new(widget.clone(), true, Default::default()) }
                .unwrap();
        *self.renderer.borrow_mut() = Some(Renderer::new(context));
    }

    fn unrealize(&self) {
        *self.renderer.borrow_mut() = None;

        self.parent_unrealize();
    }
}

impl GLAreaImpl for GliumGLArea {
    fn render(&self, _context: &gtk::gdk::GLContext) -> glib::Propagation {
        self.renderer.borrow().as_ref().unwrap().draw();

        glib::Propagation::Stop
    }
}
