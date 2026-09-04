use crate::ast::*;
use crate::diagnostics::{Diagnostic, Span};
use crate::lexer::{Token, TokenKind, lex};

type GroupBody = (String, i32, i32, Vec<Spanned<Item>>);
type AnimationBody = (String, i32, Vec<AnimationFrame>);

pub fn parse(src: &str) -> Result<Document, Diagnostic> {
    Parser {
        tokens: lex(src)?,
        pos: 0,
    }
    .document()
}

struct Parser {
    tokens: Vec<Token>,
    pos: usize,
}
impl Parser {
    fn document(&mut self) -> Result<Document, Diagnostic> {
        let mut statements = Vec::new();
        while !self.eof() {
            statements.push(self.stmt()?);
        }
        Ok(Document { statements })
    }
    fn stmt(&mut self) -> Result<Spanned<Stmt>, Diagnostic> {
        let start = self.peek().span;
        let kw = self.ident()?;
        let node = match kw.as_str() {
            "canvas" => Stmt::Canvas {
                width: self.number()?,
                height: self.number()?,
            },
            "background" => Stmt::Background {
                color: self.color_expr()?,
            },
            "strict_palette" => Stmt::StrictPalette {
                value: self.bool()?,
            },
            "palette" => Stmt::Palette {
                entries: self.palette_entries()?,
            },
            "group" => {
                let (name, x, y, items) = self.group_body()?;
                Stmt::Group { name, x, y, items }
            }
            "frame" => Stmt::Frame {
                name: self.ident()?,
                items: self.item_block()?,
            },
            "animation" => {
                let (name, fps, frames) = self.animation_body()?;
                Stmt::Animation { name, fps, frames }
            }
            "pixel" | "line" | "rect" | "fill" | "circle" => Stmt::Item(self.item_after_kw(kw)?),
            _ => {
                return Err(Diagnostic::new(
                    "E004",
                    start,
                    format!("unknown statement `{kw}`"),
                ));
            }
        };
        Ok(Spanned::new(node, start))
    }
    fn item(&mut self) -> Result<Spanned<Item>, Diagnostic> {
        let start = self.peek().span;
        let kw = self.ident()?;
        let node = self.item_after_kw(kw)?;
        Ok(Spanned::new(node, start))
    }
    fn item_after_kw(&mut self, kw: String) -> Result<Item, Diagnostic> {
        Ok(match kw.as_str() {
            "pixel" => Item::Pixel {
                x: self.number()?,
                y: self.number()?,
                color: self.color_expr()?,
            },
            "line" => Item::Line {
                x1: self.number()?,
                y1: self.number()?,
                x2: self.number()?,
                y2: self.number()?,
                color: self.color_expr()?,
            },
            "rect" => Item::Rect {
                x: self.number()?,
                y: self.number()?,
                w: self.number()?,
                h: self.number()?,
                color: self.color_expr()?,
            },
            "fill" => Item::Fill {
                x: self.number()?,
                y: self.number()?,
                color: self.color_expr()?,
            },
            "circle" => Item::Circle {
                cx: self.number()?,
                cy: self.number()?,
                r: self.number()?,
                color: self.color_expr()?,
            },
            "group" => {
                let (name, x, y, items) = self.group_body()?;
                Item::Group { name, x, y, items }
            }
            _ => {
                return Err(Diagnostic::new(
                    "E004",
                    self.prev_span(),
                    format!("unknown item `{kw}`"),
                ));
            }
        })
    }
    fn palette_entries(&mut self) -> Result<Vec<PaletteEntry>, Diagnostic> {
        self.lbrace()?;
        let mut entries = Vec::new();
        while !self.check_rbrace() {
            let span = self.peek().span;
            let name = self.ident()?;
            let color = self.color_expr()?;
            entries.push(PaletteEntry { name, color, span });
        }
        self.rbrace()?;
        Ok(entries)
    }
    fn group_body(&mut self) -> Result<GroupBody, Diagnostic> {
        let name = self.ident()?;
        let at = self.ident()?;
        if at != "at" {
            return Err(Diagnostic::new("E005", self.prev_span(), "expected `at`"));
        }
        let x = self.number()?;
        let y = self.number()?;
        let items = self.item_block()?;
        Ok((name, x, y, items))
    }
    fn item_block(&mut self) -> Result<Vec<Spanned<Item>>, Diagnostic> {
        self.lbrace()?;
        let mut items = Vec::new();
        while !self.check_rbrace() {
            items.push(self.item()?);
        }
        self.rbrace()?;
        Ok(items)
    }
    fn animation_body(&mut self) -> Result<AnimationBody, Diagnostic> {
        let name = self.ident()?;
        let fps_kw = self.ident()?;
        if fps_kw != "fps" {
            return Err(Diagnostic::new("E012", self.prev_span(), "expected `fps`"));
        }
        let fps = self.number()?;
        self.lbrace()?;
        let mut frames = Vec::new();
        while !self.check_rbrace() {
            let span = self.peek().span;
            frames.push(AnimationFrame {
                name: self.ident()?,
                span,
            });
        }
        self.rbrace()?;
        Ok((name, fps, frames))
    }
    fn color_expr(&mut self) -> Result<ColorExpr, Diagnostic> {
        let t = self.advance().clone();
        match t.kind {
            TokenKind::Ident(s) | TokenKind::Color(s) => Ok(ColorExpr {
                raw: s,
                span: t.span,
            }),
            _ => Err(Diagnostic::new("E006", t.span, "expected color")),
        }
    }
    fn bool(&mut self) -> Result<bool, Diagnostic> {
        let span = self.peek().span;
        let s = self.ident()?;
        match s.as_str() {
            "true" => Ok(true),
            "false" => Ok(false),
            _ => Err(Diagnostic::new("E007", span, "expected `true` or `false`")),
        }
    }
    fn ident(&mut self) -> Result<String, Diagnostic> {
        let t = self.advance().clone();
        match t.kind {
            TokenKind::Ident(s) => Ok(s),
            _ => Err(Diagnostic::new("E008", t.span, "expected identifier")),
        }
    }
    fn number(&mut self) -> Result<i32, Diagnostic> {
        let t = self.advance().clone();
        match t.kind {
            TokenKind::Number(n) => Ok(n),
            _ => Err(Diagnostic::new("E009", t.span, "expected number")),
        }
    }
    fn lbrace(&mut self) -> Result<(), Diagnostic> {
        let t = self.advance().clone();
        if matches!(t.kind, TokenKind::LBrace) {
            Ok(())
        } else {
            Err(Diagnostic::new("E010", t.span, "expected `{`"))
        }
    }
    fn rbrace(&mut self) -> Result<(), Diagnostic> {
        let t = self.advance().clone();
        if matches!(t.kind, TokenKind::RBrace) {
            Ok(())
        } else {
            Err(Diagnostic::new("E011", t.span, "expected `}`"))
        }
    }
    fn check_rbrace(&self) -> bool {
        matches!(self.peek().kind, TokenKind::RBrace | TokenKind::Eof)
    }
    fn eof(&self) -> bool {
        matches!(self.peek().kind, TokenKind::Eof)
    }
    fn peek(&self) -> &Token {
        &self.tokens[self.pos]
    }
    fn advance(&mut self) -> &Token {
        let p = self.pos;
        if !self.eof() {
            self.pos += 1;
        }
        &self.tokens[p]
    }
    fn prev_span(&self) -> Span {
        self.tokens[self.pos.saturating_sub(1)].span
    }
}
