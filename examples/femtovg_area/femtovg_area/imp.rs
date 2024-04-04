use std::{
    cell::{Cell, RefCell},
    f32::consts::PI,
    num::NonZeroU32,
    time::Instant,
};

use gtk::{glib, prelude::*, subclass::prelude::*};

const STROKE_WIDTH: u32 = 2;
const MARGIN: u32 = 8 + STROKE_WIDTH / 2;
const DEFAULT_N: u32 = 5;
const MAX_N: u32 = 8;

pub struct FemtoVGArea {
    canvas: RefCell<Option<femtovg::Canvas<femtovg::renderer::OpenGl>>>,
    start_time: Cell<Instant>,
    n: Cell<u32>,
}

impl Default for FemtoVGArea {
    fn default() -> Self {
        Self {
            canvas: Default::default(),
            start_time: Cell::new(Instant::now()),
            n: Cell::new(DEFAULT_N),
        }
    }
}

#[glib::object_subclass]
impl ObjectSubclass for FemtoVGArea {
    const NAME: &'static str = "FemtoVGArea";
    type Type = super::FemtoVGArea;
    type ParentType = gtk::GLArea;
}

impl ObjectImpl for FemtoVGArea {
    fn constructed(&self) {
        self.parent_constructed();
        let area = self.obj();
        area.set_has_stencil_buffer(true);
        area.add_tick_callback(|area, _| {
            area.queue_render();
            glib::ControlFlow::Continue
        });
        let click = gtk::GestureClick::new();
        let a = area.clone();
        click.connect_pressed(move |_, _, _, _| {
            let imp = a.imp();
            let mut n = imp.n.get() + 1;
            if n > MAX_N {
                n = DEFAULT_N;
            }
            imp.n.set(n);
        });
        area.add_controller(click);
    }
}

impl WidgetImpl for FemtoVGArea {
    fn realize(&self) {
        self.parent_realize();
        self.start_time.set(Instant::now());
    }
    fn unrealize(&self) {
        self.obj().make_current();
        self.canvas.replace(None);
        self.parent_unrealize();
    }
}

impl GLAreaImpl for FemtoVGArea {
    fn resize(&self, width: i32, height: i32) {
        self.ensure_canvas();
        let mut canvas = self.canvas.borrow_mut();
        let canvas = canvas.as_mut().unwrap();
        canvas.set_size(
            width as u32,
            height as u32,
            self.obj().scale_factor() as f32,
        );
    }
    fn render(&self, _context: &gtk::gdk::GLContext) -> glib::Propagation {
        use femtovg::{Color, Paint, Path};

        self.ensure_canvas();
        let mut canvas = self.canvas.borrow_mut();
        let canvas = canvas.as_mut().unwrap();

        let area = self.obj();
        let w = area.width() as u32;
        let h = area.height() as u32;
        canvas.reset_transform();
        canvas.clear_rect(0, 0, w, h, Color::rgba(0, 0, 0, 0));

        let dim = w.min(h).saturating_sub(MARGIN * 2);
        let dh = dim as f32 / 2.;
        let range = 0..self.n.get();
        let n = range.end as f32;
        let ir = ((PI * 2.) / n).cos() / (PI / n).cos();

        canvas.translate(w as f32 / 2., h as f32 / 2.);
        canvas.rotate(self.start_time.get().elapsed().as_secs_f32());

        let mut path = Path::new();
        for i in range {
            let (px, py) = (PI / n * (i as f32 - 0.5) * 2.).sin_cos();
            let (px, py) = (px * ir * dh, -py * ir * dh);
            if i == 0 {
                path.move_to(px, py);
            } else {
                path.line_to(px, py);
            }
            let (px, py) = (PI / n * (i as f32) * 2.).sin_cos();
            path.line_to(px * dh, -py * dh);
        }
        path.close();

        let r = (n - DEFAULT_N as f32) / (MAX_N - DEFAULT_N) as f32;
        canvas.fill_path(
            &mut path,
            &Paint::color(Color::rgba((r * 255.) as u8, 0, 255, 255)),
        );
        let mut paint = Paint::color(Color::rgba(0, 0, 0, 255));
        paint.set_line_width(2.);
        canvas.stroke_path(&mut path, &paint);
        canvas.flush();

        glib::Propagation::Stop
    }
}
impl FemtoVGArea {
    fn ensure_canvas(&self) {
        use femtovg::{renderer, Canvas};
        use glow::HasContext;

        if self.canvas.borrow().is_some() {
            return;
        }
        let widget = self.obj();
        widget.attach_buffers();

        static LOAD_FN: fn(&str) -> *const std::ffi::c_void =
            |s| epoxy::get_proc_addr(s) as *const _;
        // SAFETY: Need to get the framebuffer id that gtk expects us to draw into, so
        // femtovg knows which framebuffer to bind. This is safe as long as we
        // call attach_buffers beforehand. Also unbind it here just in case,
        // since this can be called outside render.
        let (mut renderer, fbo) = unsafe {
            let renderer =
                renderer::OpenGl::new_from_function(LOAD_FN).expect("Cannot create renderer");
            let ctx = glow::Context::from_loader_function(LOAD_FN);
            let id = NonZeroU32::new(ctx.get_parameter_i32(glow::DRAW_FRAMEBUFFER_BINDING) as u32)
                .expect("No GTK provided framebuffer binding");
            ctx.bind_framebuffer(glow::FRAMEBUFFER, None);
            (renderer, glow::NativeFramebuffer(id))
        };
        renderer.set_screen_target(Some(fbo));
        let canvas = Canvas::new(renderer).expect("Cannot create canvas");
        self.canvas.replace(Some(canvas));
    }
}
