use crate::ast::ColorExpr;
use crate::diagnostics::Diagnostic;
use serde::Serialize;
use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct Rgba {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}
impl Rgba {
    pub const TRANSPARENT: Rgba = Rgba {
        r: 0,
        g: 0,
        b: 0,
        a: 0,
    };
}

pub type Palette = HashMap<String, Rgba>;

pub fn parse_color_literal(raw: &str) -> Option<Rgba> {
    if raw == "transparent" {
        return Some(Rgba::TRANSPARENT);
    }
    let h = raw.strip_prefix('#')?;
    if h.len() != 6 && h.len() != 8 {
        return None;
    }
    let r = u8::from_str_radix(&h[0..2], 16).ok()?;
    let g = u8::from_str_radix(&h[2..4], 16).ok()?;
    let b = u8::from_str_radix(&h[4..6], 16).ok()?;
    let a = if h.len() == 8 {
        u8::from_str_radix(&h[6..8], 16).ok()?
    } else {
        255
    };
    Some(Rgba { r, g, b, a })
}

pub fn resolve_color(
    expr: &ColorExpr,
    palette: &Palette,
    strict: bool,
) -> Result<Rgba, Diagnostic> {
    if let Some(c) = palette.get(&expr.raw) {
        return Ok(*c);
    }
    if let Some(c) = parse_color_literal(&expr.raw) {
        if strict && expr.raw != "transparent" && !palette.values().any(|p| *p == c) {
            return Err(Diagnostic::new(
                "E013",
                expr.span,
                format!("color `{}` is outside strict palette", expr.raw),
            ));
        }
        return Ok(c);
    }
    let mut d = Diagnostic::new("E012", expr.span, format!("unknown color `{}`", expr.raw));
    if let Some(s) = suggest(&expr.raw, palette.keys()) {
        d = d.with_suggestion(s);
    }
    Err(d)
}

fn suggest<'a>(name: &str, keys: impl Iterator<Item = &'a String>) -> Option<String> {
    keys.min_by_key(|k| levenshtein(name, k))
        .filter(|k| levenshtein(name, k) <= 2)
        .cloned()
}
fn levenshtein(a: &str, b: &str) -> usize {
    let mut costs: Vec<usize> = (0..=b.len()).collect();
    for (i, ca) in a.chars().enumerate() {
        let mut last = i;
        costs[0] = i + 1;
        for (j, cb) in b.chars().enumerate() {
            let old = costs[j + 1];
            costs[j + 1] = if ca == cb {
                last
            } else {
                1 + last.min(costs[j]).min(old)
            };
            last = old;
        }
    }
    costs[b.len()]
}
