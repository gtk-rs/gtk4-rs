use std::{
    cell::{Cell, RefCell},
    f64::consts::PI,
};

use gtk::{glib, prelude::*, subclass::prelude::*};

use crate::TOTAL_COLORS;

const N_GRID_COLUMNS: u32 = 4;

#[derive(Debug)]
pub struct CustomLayout {
    pub position: Cell<f64>,
    pub child_pos: RefCell<Vec<i32>>,
}
impl Default for CustomLayout {
    fn default() -> Self {
        let positions = (0..TOTAL_COLORS).collect::<Vec<_>>();
        Self {
            position: Cell::new(0f64),
            child_pos: RefCell::new(positions),
        }
    }
}

#[glib::object_subclass]
impl ObjectSubclass for CustomLayout {
    const NAME: &'static str = "CustomLayout";
    type Type = super::CustomLayout;
    type ParentType = gtk::LayoutManager;
}

impl ObjectImpl for CustomLayout {}
impl LayoutManagerImpl for CustomLayout {
    fn request_mode(&self, _widget: &gtk::Widget) -> gtk::SizeRequestMode {
        gtk::SizeRequestMode::ConstantSize
    }

    fn measure(
        &self,
        widget: &gtk::Widget,
        orientation: gtk::Orientation,
        _for_size: i32,
    ) -> (i32, i32, i32, i32) {
        let mut min_size = 0;
        let mut nat_size = 0;

        let mut child = widget.first_child().unwrap();
        loop {
            if !child.should_layout() {
                continue;
            }

            let (child_min, child_nat, _, _) = child.measure(orientation, -1);
            min_size = min_size.max(child_min);
            nat_size = nat_size.max(child_nat);
            match child.next_sibling() { Some(next_child) => {
                child = next_child;
            } _ => {
                break;
            }}
        }

        min_size = (TOTAL_COLORS as f64 * (min_size as f64) / PI + min_size as f64) as i32;
        nat_size = (TOTAL_COLORS as f64 * (nat_size as f64) / PI + nat_size as f64) as i32;

        (min_size, nat_size, -1, -1)
    }

    #[allow(clippy::many_single_char_names)]
    fn allocate(&self, widget: &gtk::Widget, width: i32, height: i32, _baseline: i32) {
        let mut child_width = 0;
        let mut child_height = 0;
        let t = self.position.get();
        let mut child = widget.first_child().unwrap();
        loop {
            if !child.should_layout() {
                continue;
            }

            let (child_req, _) = child.preferred_size();
            child_width = child_width.max(child_req.width());
            child_height = child_height.max(child_req.height());
            match child.next_sibling() { Some(next_child) => {
                child = next_child;
            } _ => {
                break;
            }}
        }

        let x0 = width as f64 / 2.0;
        let y0 = height as f64 / 2.0;

        let radius = (TOTAL_COLORS as f64 / 2.0) * (child_width as f64) / PI;
        let mut i = 0;
        let mut child = widget.first_child().unwrap();
        loop {
            if !child.should_layout() {
                continue;
            }

            let (child_req, _) = child.preferred_size();

            let gx = x0 + (i as f64 % N_GRID_COLUMNS as f64 - 2.0) * child_width as f64;
            let gy = y0 + (i as f64 / N_GRID_COLUMNS as f64 - 2.0) * child_height as f64;

            let a = *self.child_pos.borrow().get(i as usize).unwrap() as f64
                * (PI / (TOTAL_COLORS as f64 / 2.0));

            let cx = x0 + a.sin() * radius - child_req.width() as f64 / 2.0;
            let cy = y0 + a.cos() * radius - child_req.height() as f64 / 2.0;

            let x = t * cx + (1.0 - t) * gx;
            let y = t * cy + (1.0 - t) * gy;
            child.size_allocate(
                &gtk::Allocation::new(x as i32, y as i32, child_width, child_height),
                -1,
            );
            i += 1;
            match child.next_sibling() { Some(next_child) => {
                child = next_child;
            } _ => {
                break;
            }}
        }
    }
}
