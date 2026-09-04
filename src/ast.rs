use crate::diagnostics::Span;
use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct Document {
    pub statements: Vec<Spanned<Stmt>>,
}

#[derive(Debug, Clone, Serialize)]
pub struct Spanned<T> {
    pub node: T,
    pub span: Span,
}
impl<T> Spanned<T> {
    pub fn new(node: T, span: Span) -> Self {
        Self { node, span }
    }
}

#[derive(Debug, Clone, Serialize)]
pub enum Stmt {
    Canvas {
        width: i32,
        height: i32,
    },
    Background {
        color: ColorExpr,
    },
    StrictPalette {
        value: bool,
    },
    Palette {
        entries: Vec<PaletteEntry>,
    },
    Group {
        name: String,
        x: i32,
        y: i32,
        items: Vec<Spanned<Item>>,
    },
    Frame {
        name: String,
        items: Vec<Spanned<Item>>,
    },
    Animation {
        name: String,
        fps: i32,
        frames: Vec<AnimationFrame>,
    },
    Item(Item),
}

#[derive(Debug, Clone, Serialize)]
pub struct PaletteEntry {
    pub name: String,
    pub color: ColorExpr,
    pub span: Span,
}

#[derive(Debug, Clone, Serialize)]
pub struct AnimationFrame {
    pub name: String,
    pub span: Span,
}

#[derive(Debug, Clone, Serialize)]
pub enum Item {
    Pixel {
        x: i32,
        y: i32,
        color: ColorExpr,
    },
    Line {
        x1: i32,
        y1: i32,
        x2: i32,
        y2: i32,
        color: ColorExpr,
    },
    Rect {
        x: i32,
        y: i32,
        w: i32,
        h: i32,
        color: ColorExpr,
    },
    Fill {
        x: i32,
        y: i32,
        color: ColorExpr,
    },
    Circle {
        cx: i32,
        cy: i32,
        r: i32,
        color: ColorExpr,
    },
    Group {
        name: String,
        x: i32,
        y: i32,
        items: Vec<Spanned<Item>>,
    },
}

#[derive(Debug, Clone, Serialize)]
pub struct ColorExpr {
    pub raw: String,
    pub span: Span,
}
