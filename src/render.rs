use crate::ast::{AnimationFrame, Item, Spanned, Stmt};
use crate::color::{Rgba, resolve_color};
use crate::diagnostics::Diagnostic;
use crate::validate::ValidDocument;
use image::{ImageBuffer, ImageError};
use serde::Serialize;
use std::collections::VecDeque;
use std::path::Path;

pub struct Canvas {
    width: u32,
    height: u32,
    pixels: Vec<Rgba>,
}

#[derive(Debug, Serialize)]
pub struct SheetMetadata {
    pub frame_width: u32,
    pub frame_height: u32,
    pub fps: Option<i32>,
    pub frames: Vec<SheetFrame>,
}

#[derive(Debug, Serialize)]
pub struct SheetFrame {
    pub name: String,
    pub x: u32,
    pub y: u32,
    pub w: u32,
    pub h: u32,
}

impl Canvas {
    pub fn new(width: u32, height: u32, background: Rgba) -> Self {
        Self {
            width,
            height,
            pixels: vec![background; (width * height) as usize],
        }
    }
    fn idx(&self, x: i32, y: i32) -> Option<usize> {
        if x < 0 || y < 0 || x >= self.width as i32 || y >= self.height as i32 {
            None
        } else {
            Some((y as u32 * self.width + x as u32) as usize)
        }
    }
    fn set(&mut self, x: i32, y: i32, c: Rgba) {
        if let Some(i) = self.idx(x, y) {
            self.pixels[i] = c;
        }
    }
    fn get(&self, x: i32, y: i32) -> Option<Rgba> {
        self.idx(x, y).map(|i| self.pixels[i])
    }
    pub fn save_png(&self, path: &Path, scale: u32) -> Result<(), ImageError> {
        let scale = scale.max(1);
        let w = self.width * scale;
        let h = self.height * scale;
        let mut img = ImageBuffer::<image::Rgba<u8>, Vec<u8>>::new(w, h);
        for y in 0..h {
            for x in 0..w {
                let p = self.pixels[((y / scale) * self.width + (x / scale)) as usize];
                img.put_pixel(x, y, image::Rgba([p.r, p.g, p.b, p.a]));
            }
        }
        img.save(path)
    }
}

pub fn render(doc: &ValidDocument) -> Result<Canvas, Diagnostic> {
    let mut canvas = Canvas::new(doc.canvas.0, doc.canvas.1, doc.background);
    for stmt in &doc.document.statements {
        match &stmt.node {
            Stmt::Item(item) => render_item(&mut canvas, doc, item, 0, 0)?,
            Stmt::Group { x, y, items, .. } => render_items(&mut canvas, doc, items, *x, *y)?,
            _ => {}
        }
    }
    Ok(canvas)
}

pub fn render_frame(doc: &ValidDocument, frame_name: &str) -> Result<Canvas, Diagnostic> {
    for stmt in &doc.document.statements {
        if let Stmt::Frame { name, items } = &stmt.node
            && name == frame_name
        {
            let mut canvas = Canvas::new(doc.canvas.0, doc.canvas.1, doc.background);
            render_items(&mut canvas, doc, items, 0, 0)?;
            return Ok(canvas);
        }
    }
    Err(Diagnostic::new(
        "E031",
        crate::diagnostics::Span::new(1, 1, 1),
        format!("unknown frame `{frame_name}`"),
    ))
}

pub fn animation_frames(
    doc: &ValidDocument,
    animation_name: &str,
) -> Result<(i32, Vec<AnimationFrame>), Diagnostic> {
    for stmt in &doc.document.statements {
        if let Stmt::Animation { name, fps, frames } = &stmt.node
            && name == animation_name
        {
            return Ok((*fps, frames.clone()));
        }
    }
    Err(Diagnostic::new(
        "E032",
        crate::diagnostics::Span::new(1, 1, 1),
        format!("unknown animation `{animation_name}`"),
    ))
}

pub fn render_animation(
    doc: &ValidDocument,
    animation_name: &str,
) -> Result<(i32, Vec<(String, Canvas)>), Diagnostic> {
    let (fps, frames) = animation_frames(doc, animation_name)?;
    let mut rendered = Vec::new();
    for frame in frames {
        rendered.push((frame.name.clone(), render_frame(doc, &frame.name)?));
    }
    Ok((fps, rendered))
}

pub fn save_spritesheet(
    frames: &[(String, Canvas)],
    path: &Path,
    scale: u32,
    columns: u32,
    fps: Option<i32>,
) -> Result<SheetMetadata, ImageError> {
    if frames.is_empty() {
        let metadata = SheetMetadata {
            frame_width: 0,
            frame_height: 0,
            fps,
            frames: Vec::new(),
        };
        return Ok(metadata);
    }

    let scale = scale.max(1);
    let columns = columns.max(1).min(frames.len() as u32);
    let rows = (frames.len() as u32).div_ceil(columns);
    let frame_width = frames[0].1.width;
    let frame_height = frames[0].1.height;
    let mut img = ImageBuffer::<image::Rgba<u8>, Vec<u8>>::new(
        frame_width * columns * scale,
        frame_height * rows * scale,
    );
    let mut sheet_frames = Vec::new();

    for (i, (name, canvas)) in frames.iter().enumerate() {
        let col = i as u32 % columns;
        let row = i as u32 / columns;
        let dst_x = col * frame_width;
        let dst_y = row * frame_height;
        for y in 0..(frame_height * scale) {
            for x in 0..(frame_width * scale) {
                let p = canvas.pixels[((y / scale) * frame_width + (x / scale)) as usize];
                img.put_pixel(
                    dst_x * scale + x,
                    dst_y * scale + y,
                    image::Rgba([p.r, p.g, p.b, p.a]),
                );
            }
        }
        sheet_frames.push(SheetFrame {
            name: name.clone(),
            x: dst_x * scale,
            y: dst_y * scale,
            w: frame_width * scale,
            h: frame_height * scale,
        });
    }

    img.save(path)?;
    Ok(SheetMetadata {
        frame_width: frame_width * scale,
        frame_height: frame_height * scale,
        fps,
        frames: sheet_frames,
    })
}

fn render_items(
    canvas: &mut Canvas,
    doc: &ValidDocument,
    items: &[Spanned<Item>],
    ox: i32,
    oy: i32,
) -> Result<(), Diagnostic> {
    for item in items {
        render_item(canvas, doc, &item.node, ox, oy)?;
    }
    Ok(())
}

fn render_item(
    canvas: &mut Canvas,
    doc: &ValidDocument,
    item: &Item,
    ox: i32,
    oy: i32,
) -> Result<(), Diagnostic> {
    match item {
        Item::Pixel { x, y, color } => canvas.set(
            ox + x,
            oy + y,
            resolve_color(color, &doc.palette, doc.strict_palette)?,
        ),
        Item::Line {
            x1,
            y1,
            x2,
            y2,
            color,
        } => line(
            canvas,
            ox + x1,
            oy + y1,
            ox + x2,
            oy + y2,
            resolve_color(color, &doc.palette, doc.strict_palette)?,
        ),
        Item::Rect { x, y, w, h, color } => {
            let c = resolve_color(color, &doc.palette, doc.strict_palette)?;
            for yy in 0..*h {
                for xx in 0..*w {
                    canvas.set(ox + x + xx, oy + y + yy, c);
                }
            }
        }
        Item::Fill { x, y, color } => fill(
            canvas,
            ox + x,
            oy + y,
            resolve_color(color, &doc.palette, doc.strict_palette)?,
        ),
        Item::Circle { cx, cy, r, color } => circle(
            canvas,
            ox + cx,
            oy + cy,
            *r,
            resolve_color(color, &doc.palette, doc.strict_palette)?,
        ),
        Item::Group { x, y, items, .. } => render_items(canvas, doc, items, ox + x, oy + y)?,
    }
    Ok(())
}

fn line(canvas: &mut Canvas, mut x0: i32, mut y0: i32, x1: i32, y1: i32, c: Rgba) {
    let dx = (x1 - x0).abs();
    let sx = if x0 < x1 { 1 } else { -1 };
    let dy = -(y1 - y0).abs();
    let sy = if y0 < y1 { 1 } else { -1 };
    let mut err = dx + dy;
    loop {
        canvas.set(x0, y0, c);
        if x0 == x1 && y0 == y1 {
            break;
        }
        let e2 = 2 * err;
        if e2 >= dy {
            err += dy;
            x0 += sx;
        }
        if e2 <= dx {
            err += dx;
            y0 += sy;
        }
    }
}
fn circle(canvas: &mut Canvas, cx: i32, cy: i32, r: i32, c: Rgba) {
    let rr = r * r;
    for y in -r..=r {
        for x in -r..=r {
            if x * x + y * y <= rr {
                canvas.set(cx + x, cy + y, c);
            }
        }
    }
}
fn fill(canvas: &mut Canvas, x: i32, y: i32, c: Rgba) {
    let Some(target) = canvas.get(x, y) else {
        return;
    };
    if target == c {
        return;
    }
    let mut q = VecDeque::from([(x, y)]);
    while let Some((px, py)) = q.pop_front() {
        if canvas.get(px, py) != Some(target) {
            continue;
        }
        canvas.set(px, py, c);
        q.extend([(px + 1, py), (px - 1, py), (px, py + 1), (px, py - 1)]);
    }
}
