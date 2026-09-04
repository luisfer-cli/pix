use crate::ast::{ColorExpr, Document, Item, Spanned, Stmt};

pub fn format_document(doc: &Document) -> String {
    let mut out = String::new();
    for stmt in &doc.statements {
        fmt_stmt(&mut out, stmt, 0);
        out.push('\n');
    }
    out
}

fn indent(out: &mut String, n: usize) {
    out.push_str(&"    ".repeat(n));
}
fn color(c: &ColorExpr) -> &str {
    &c.raw
}

fn fmt_stmt(out: &mut String, stmt: &Spanned<Stmt>, level: usize) {
    indent(out, level);
    match &stmt.node {
        Stmt::Canvas { width, height } => out.push_str(&format!("canvas {width} {height}\n")),
        Stmt::Background { color: c } => out.push_str(&format!("background {}\n", color(c))),
        Stmt::StrictPalette { value } => out.push_str(&format!("strict_palette {value}\n")),
        Stmt::Palette { entries } => {
            out.push_str("palette {\n");
            for e in entries {
                indent(out, level + 1);
                out.push_str(&format!("{} {}\n", e.name, color(&e.color)));
            }
            indent(out, level);
            out.push_str("}\n");
        }
        Stmt::Group { name, x, y, items } => fmt_group(out, name, *x, *y, items, level),
        Stmt::Frame { name, items } => fmt_frame(out, name, items, level),
        Stmt::Animation { name, fps, frames } => {
            out.push_str(&format!("animation {name} fps {fps} {{\n"));
            for frame in frames {
                indent(out, level + 1);
                out.push_str(&format!("{}\n", frame.name));
            }
            indent(out, level);
            out.push_str("}\n");
        }
        Stmt::Item(item) => {
            fmt_item(out, item, level);
            out.push('\n');
        }
    }
}
fn fmt_group(out: &mut String, name: &str, x: i32, y: i32, items: &[Spanned<Item>], level: usize) {
    out.push_str(&format!("group {name} at {x} {y} {{\n"));
    fmt_items(out, items, level);
    indent(out, level);
    out.push_str("}\n");
}
fn fmt_frame(out: &mut String, name: &str, items: &[Spanned<Item>], level: usize) {
    out.push_str(&format!("frame {name} {{\n"));
    fmt_items(out, items, level);
    indent(out, level);
    out.push_str("}\n");
}
fn fmt_items(out: &mut String, items: &[Spanned<Item>], level: usize) {
    for item in items {
        indent(out, level + 1);
        fmt_item(out, &item.node, level + 1);
        out.push('\n');
    }
}
fn fmt_item(out: &mut String, item: &Item, level: usize) {
    match item {
        Item::Pixel { x, y, color: c } => out.push_str(&format!("pixel {x} {y} {}", color(c))),
        Item::Line {
            x1,
            y1,
            x2,
            y2,
            color: c,
        } => out.push_str(&format!("line {x1} {y1} {x2} {y2} {}", color(c))),
        Item::Rect {
            x,
            y,
            w,
            h,
            color: c,
        } => out.push_str(&format!("rect {x} {y} {w} {h} {}", color(c))),
        Item::Fill { x, y, color: c } => out.push_str(&format!("fill {x} {y} {}", color(c))),
        Item::Circle {
            cx,
            cy,
            r,
            color: c,
        } => out.push_str(&format!("circle {cx} {cy} {r} {}", color(c))),
        Item::Group { name, x, y, items } => fmt_group(out, name, *x, *y, items, level),
    }
}
