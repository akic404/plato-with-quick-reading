use crate::framebuffer::Framebuffer;
use super::{View, Event, Hub, Bus, Id, ID_FEEDER, RenderQueue, ViewId};
use crate::geom::Rectangle;
use crate::context::Context;
use crate::color::Color;
use crate::font::Fonts;

const LINE_HEIGHT_RATIO: f32 = 0.9;
const LINE_TOP_RATIO: f32 = 0.05;
pub const MARGIN_SHIFT: f32 = 0.03;
pub const MARGIN_CAP: f32 = 0.37;

#[derive(Debug)]
pub struct PerceptionExpander {
    id: Id,
    rect: Rectangle,
    children: Vec<Box<dyn View>>,
    line_thickness: i32,
    margin: f32,
    intensity: i32,
    shift_each_pages: i32,
    page_counter: i32,
}

impl PerceptionExpander {
    pub fn new(rect: Rectangle, line_thickness: i32, margin: f32, intensity: i32,
               shift_each_pages: i32, page_counter: i32) -> PerceptionExpander {
        PerceptionExpander {
            id: ID_FEEDER.next(),
            rect,
            children: Vec::new(),
            line_thickness,
            margin,
            intensity,
            shift_each_pages,
            page_counter,
        }
    }

    pub fn margin(&self) -> f32 {
        self.margin
    }

    pub fn page_counter(&self) -> i32 {
        self.page_counter
    }

    pub fn set_line_thickness(&mut self, line_thickness: i32) {
        self.line_thickness = line_thickness;
    }

    pub fn set_margin(&mut self, margin: f32) {
        self.margin = margin;
    }

    pub fn set_intensity(&mut self, intensity: i32) {
        self.intensity = intensity;
    }

    pub fn set_shift_each_pages(&mut self, shift_each_pages: i32) {
        self.shift_each_pages = shift_each_pages;
    }

    // Called once per page turn. Returns true when the margin has been shifted,
    // meaning the caller must persist the new state and redraw.
    pub fn note_page_turn(&mut self) -> bool {
        self.page_counter += 1;
        if self.shift_each_pages > 0 && self.page_counter >= self.shift_each_pages
           && self.margin < MARGIN_CAP {
            self.page_counter = 0;
            self.margin = (self.margin + MARGIN_SHIFT).min(MARGIN_CAP);
            true
        } else {
            false
        }
    }

    fn line_rects(&self) -> [Rectangle; 2] {
        let height = self.rect.height() as f32;
        let line_height = (height * LINE_HEIGHT_RATIO) as i32;
        let top = self.rect.min.y + (height * LINE_TOP_RATIO) as i32;
        let width = self.rect.width() as f32;
        let xl = self.rect.min.x + (width * self.margin) as i32;
        let xr = self.rect.max.x - (width * self.margin) as i32 - self.line_thickness;
        [rect![xl, top, xl + self.line_thickness, top + line_height],
         rect![xr, top, xr + self.line_thickness, top + line_height]]
    }

    fn line_color(&self) -> Color {
        // 1 (light gray) … 10 (black)
        let level = (15 - self.intensity.clamp(1, 10) * 3 / 2).max(0) * 0x11;
        Color::Gray(level as u8)
    }
}

impl View for PerceptionExpander {
    fn handle_event(&mut self, _evt: &Event, _hub: &Hub, _bus: &mut Bus, _rq: &mut RenderQueue, _context: &mut Context) -> bool {
        false
    }

    fn render(&self, fb: &mut dyn Framebuffer, rect: Rectangle, _fonts: &mut Fonts) {
        for line in self.line_rects() {
            if let Some(r) = line.intersection(&rect) {
                fb.draw_rectangle(&r, self.line_color());
            }
        }
    }

    fn render_rect(&self, rect: &Rectangle) -> Rectangle {
        rect.intersection(&self.rect)
            .unwrap_or(self.rect)
    }

    fn rect(&self) -> &Rectangle {
        &self.rect
    }

    fn rect_mut(&mut self) -> &mut Rectangle {
        &mut self.rect
    }

    fn children(&self) -> &Vec<Box<dyn View>> {
        &self.children
    }

    fn children_mut(&mut self) -> &mut Vec<Box<dyn View>> {
        &mut self.children
    }

    fn id(&self) -> Id {
        self.id
    }

    fn view_id(&self) -> Option<ViewId> {
        Some(ViewId::PerceptionExpander)
    }
}
