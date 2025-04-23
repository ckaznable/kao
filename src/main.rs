#![feature(iter_array_chunks)]

use std::rc::Rc;

use anyhow::{anyhow, Result};
use face::{ORI_SVG_HEIGHT, ORI_SVG_WIDTH};
use ratatui::layout::Rect;
use resvg::usvg::{self, Transform, Tree};
use resvg::tiny_skia::{self, Pixmap};
use crossterm::event::{self, Event};
use ratatui::{DefaultTerminal, Frame};
use widget::face::FaceScreen;

mod face;
mod widget;

const CACHE_SIZE: usize = 5;

#[derive(Clone, Debug)]
struct PixmapCache {
    pixmap: Pixmap,
    area: Rect,
    face: face::Face,
}

#[derive(Clone, Debug)]
struct FacePixmapStore {
    caches: [Option<Rc<PixmapCache>>; CACHE_SIZE],
    index: u8,
}

impl FacePixmapStore {
    fn get(&mut self, face: face::Face, area: Rect) -> Option<Rc<PixmapCache>> {
        if let Some(cache) = self.get_from_cache(face, area) {
            return Some(cache);
        }

        let pixmap = Self::get_svg_pixmap(face.to_svg(), area.width as u32, area.height as u32 * 2).ok()?;
        let cache = Rc::new(PixmapCache { pixmap, area, face });

        self.caches[self.index as usize] = Some(cache.clone());
        self.index = if self.index == CACHE_SIZE as u8 { 0 } else { self.index + 1 };

        Some(cache)
    }

    fn get_from_cache(&self, face: face::Face, area: Rect) -> Option<Rc<PixmapCache>> {
        self.caches
            .iter()
            .find(|cache| match cache {
                Some(cache) => cache.face == face && cache.area == area,
                None => false,
            })
            .map(|cache| cache.clone())
            .flatten()
    }

    fn get_svg_pixmap<T: AsRef<str>>(svg_data: T, width: u32, height: u32) -> Result<Pixmap> {
        let opt = usvg::Options::default();
        let tree = Tree::from_str(svg_data.as_ref(), &opt)?;

        let transform = Transform::from_scale(width as f32 / ORI_SVG_WIDTH as f32, height as f32 / ORI_SVG_HEIGHT as f32);
        let mut pixmap = tiny_skia::Pixmap::new(width, height)
            .ok_or(anyhow!("can't create pixmap"))?;

        resvg::render(&tree, transform, &mut pixmap.as_mut());
        Ok(pixmap)
    }
}

impl Default for FacePixmapStore {
    fn default() -> Self {
        Self {
            caches: [const { None }; 5],
            index: 0,
        }
    }
}

fn main() -> Result<()> {
    let terminal = ratatui::init();
    let result = run(terminal);
    ratatui::restore();
    result
}

fn run(mut terminal: DefaultTerminal) -> Result<()> {
    let mut caches = FacePixmapStore::default();

    loop {
        terminal.draw(|frame| render(frame, &mut caches))?;
        if matches!(event::read()?, Event::Key(_)) {
            break Ok(());
        }
    }
}

fn render(frame: &mut Frame, caches: &mut FacePixmapStore) {
    let area = frame.area();
    let Some(pixmap) = caches.get(face::Face::default(), area) else {
        return;
    };

    frame.render_widget_ref(FaceScreen::new(pixmap.pixmap.as_ref()), area);
}
