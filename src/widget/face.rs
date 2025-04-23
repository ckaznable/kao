use ratatui::{
  buffer::Buffer,
  layout::Rect,
  style::Color,
  widgets::{
    Widget,
    WidgetRef
  }
};
use resvg::tiny_skia::PixmapRef;

pub struct FaceScreen<'a> {
  pixmap: PixmapRef<'a>
}

impl<'a> FaceScreen<'a> {
  pub fn new(pixmap: PixmapRef<'a>) -> Self {
    Self { pixmap }
  }
}

impl<'a> WidgetRef for FaceScreen<'a> {
  fn render_ref(&self, area: Rect, buf: &mut Buffer) {
    let left = area.left();
    let top = area.top();
    let width = area.width;

    self.pixmap
      .pixels()
      .chunks_exact(width as usize)
      .array_chunks()
      .map(|[a, b]| a.iter().zip(b.iter()))
      .flatten()
      .enumerate()
      .for_each(|(i, (fg, bg))| {
        let is_fg_transparent = fg.alpha() == 0;
        let is_bg_transparent = bg.alpha() == 0;

        if is_fg_transparent && is_bg_transparent {
          return;
        }

        let x = left + i as u16 % width;
        let y = top + i as u16 / width;

        let Some(cell) = buf.cell_mut((x, y)) else {
          return;
        };

        cell.set_char('▀');

        if !is_fg_transparent {
          cell.set_fg(Color::Rgb(fg.red(), fg.green(), fg.blue()));
        }

        if !is_bg_transparent {
          cell.set_bg(Color::Rgb(bg.red(), bg.green(), bg.blue()));
        }
      });
  }
}

impl<'a> Widget for FaceScreen<'a> {
  fn render(self, area: Rect, buf: &mut Buffer) {
    self.render_ref(area, buf);
  }
}
