use anyhow::{anyhow, Result};
use face::{ORI_SVG_HEIGHT, ORI_SVG_WIDTH};
use resvg::usvg::{self, Transform, Tree};
use resvg::tiny_skia::{self, Pixmap};
use crossterm::event::{self, Event};
use ratatui::{DefaultTerminal, Frame};

mod face;

fn main() -> Result<()> {
    let terminal = ratatui::init();
    let result = run(terminal);
    ratatui::restore();
    result
}

fn run(mut terminal: DefaultTerminal) -> Result<()> {
    loop {
        terminal.draw(render)?;
        if matches!(event::read()?, Event::Key(_)) {
            break Ok(());
        }
    }
}

fn render(frame: &mut Frame) {
    frame.render_widget("hello world", frame.area());
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

