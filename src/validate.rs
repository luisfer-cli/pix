use crate::ast::*;
use crate::color::{Palette, Rgba, parse_color_literal, resolve_color};
use crate::diagnostics::{Diagnostic, Span};
use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone)]
pub struct ValidDocument {
    pub canvas: (u32, u32),
    pub background: Rgba,
    pub strict_palette: bool,
    pub palette: Palette,
    pub document: Document,
}

pub fn validate(document: Document) -> Result<ValidDocument, Vec<Diagnostic>> {
    let mut errors = Vec::new();
    let mut canvas = None;
    let mut background = Rgba::TRANSPARENT;
    let mut strict = false;
    let mut palette = Palette::new();
    let mut names = HashSet::new();
    let mut frames = HashMap::new();

    for stmt in &document.statements {
        match &stmt.node {
            Stmt::Canvas { width, height } => {
                if *width <= 0 || *height <= 0 {
                    errors.push(Diagnostic::new(
                        "E020",
                        stmt.span,
                        "canvas dimensions must be positive",
                    ));
                } else {
                    canvas = Some((*width as u32, *height as u32));
                }
            }
            Stmt::StrictPalette { value } => strict = *value,
            Stmt::Palette { entries } => {
                for e in entries {
                    if !names.insert(e.name.clone()) {
                        errors.push(Diagnostic::new(
                            "E021",
                            e.span,
                            format!("duplicate palette color `{}`", e.name),
                        ));
                    }
                    match parse_color_literal(&e.color.raw) {
                        Some(c) => {
                            palette.insert(e.name.clone(), c);
                        }
                        None => errors.push(Diagnostic::new(
                            "E022",
                            e.color.span,
                            format!("invalid color literal `{}`", e.color.raw),
                        )),
                    }
                }
            }
            _ => {}
        }
    }

    if canvas.is_none() {
        errors.push(Diagnostic::new(
            "E023",
            Span::new(1, 1, 1),
            "missing `canvas WIDTH HEIGHT`",
        ));
    }

    for stmt in &document.statements {
        match &stmt.node {
            Stmt::Background { color } => match resolve_color(color, &palette, strict) {
                Ok(c) => background = c,
                Err(e) => errors.push(e),
            },
            Stmt::Group { name, items, .. } => {
                if !names.insert(name.clone()) {
                    errors.push(Diagnostic::new(
                        "E024",
                        stmt.span,
                        format!("duplicate name `{name}`"),
                    ));
                }
                validate_items(items, &palette, strict, &mut names, &mut errors);
            }
            Stmt::Frame { name, items } => {
                if !names.insert(name.clone()) {
                    errors.push(Diagnostic::new(
                        "E024",
                        stmt.span,
                        format!("duplicate name `{name}`"),
                    ));
                }
                if frames.insert(name.clone(), stmt.span).is_some() {
                    errors.push(Diagnostic::new(
                        "E027",
                        stmt.span,
                        format!("duplicate frame `{name}`"),
                    ));
                }
                let mut frame_names = names.clone();
                validate_items(items, &palette, strict, &mut frame_names, &mut errors);
            }
            Stmt::Animation {
                name,
                fps,
                frames: animation_frames,
            } => {
                if !names.insert(name.clone()) {
                    errors.push(Diagnostic::new(
                        "E024",
                        stmt.span,
                        format!("duplicate name `{name}`"),
                    ));
                }
                if *fps <= 0 {
                    errors.push(Diagnostic::new(
                        "E028",
                        stmt.span,
                        "animation fps must be positive",
                    ));
                }
                if animation_frames.is_empty() {
                    errors.push(Diagnostic::new(
                        "E029",
                        stmt.span,
                        "animation must reference at least one frame",
                    ));
                }
            }
            Stmt::Item(item) => {
                validate_item(item, stmt.span, &palette, strict, &mut names, &mut errors)
            }
            _ => {}
        }
    }

    for stmt in &document.statements {
        if let Stmt::Animation {
            frames: animation_frames,
            ..
        } = &stmt.node
        {
            for frame in animation_frames {
                if !frames.contains_key(&frame.name) {
                    errors.push(Diagnostic::new(
                        "E030",
                        frame.span,
                        format!("unknown frame `{}`", frame.name),
                    ));
                }
            }
        }
    }

    if errors.is_empty() {
        Ok(ValidDocument {
            canvas: canvas.unwrap(),
            background,
            strict_palette: strict,
            palette,
            document,
        })
    } else {
        Err(errors)
    }
}

fn validate_items(
    items: &[Spanned<Item>],
    palette: &Palette,
    strict: bool,
    names: &mut HashSet<String>,
    errors: &mut Vec<Diagnostic>,
) {
    for item in items {
        validate_item(&item.node, item.span, palette, strict, names, errors);
    }
}
fn validate_item(
    item: &Item,
    span: Span,
    palette: &Palette,
    strict: bool,
    names: &mut HashSet<String>,
    errors: &mut Vec<Diagnostic>,
) {
    match item {
        Item::Pixel { color, .. }
        | Item::Line { color, .. }
        | Item::Rect { color, .. }
        | Item::Fill { color, .. }
        | Item::Circle { color, .. } => {
            if let Err(e) = resolve_color(color, palette, strict) {
                errors.push(e);
            }
        }
        Item::Group { name, items, .. } => {
            if !names.insert(name.clone()) {
                errors.push(Diagnostic::new(
                    "E024",
                    span,
                    format!("duplicate name `{name}`"),
                ));
            }
            validate_items(items, palette, strict, names, errors);
        }
    }
    if let Item::Rect { w, h, .. } = item
        && (*w <= 0 || *h <= 0)
    {
        errors.push(Diagnostic::new(
            "E025",
            span,
            "rect width and height must be positive",
        ));
    }
    if let Item::Circle { r, .. } = item
        && *r < 0
    {
        errors.push(Diagnostic::new(
            "E026",
            span,
            "circle radius must be non-negative",
        ));
    }
}
