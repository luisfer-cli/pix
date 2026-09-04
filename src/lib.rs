pub mod ast;
pub mod cli;
pub mod color;
pub mod diagnostics;
pub mod fmt;
pub mod lexer;
pub mod parser;
pub mod render;
pub mod source;
pub mod spec;
pub mod validate;

#[cfg(test)]
mod tests {
    use super::*;

    const SLIME: &str = r#"
canvas 16 16
background transparent
strict_palette true
palette {
    O #181425
    G #3fa34d
    L #7bd88f
}
group slime at 3 5 {
    rect 0 1 10 5 G
    pixel 2 2 O
    pixel 7 2 O
    circle 5 3 2 L
}
"#;

    #[test]
    fn parses_and_validates_phase1() {
        let doc = parser::parse(SLIME).unwrap();
        let valid = validate::validate(doc).unwrap();
        assert_eq!(valid.canvas, (16, 16));
    }

    #[test]
    fn unknown_color_is_diagnostic() {
        let src = "canvas 8 8\npixel 1 1 gren\n";
        let doc = parser::parse(src).unwrap();
        let errs = validate::validate(doc).unwrap_err();
        assert_eq!(errs[0].code, "E012");
    }

    #[test]
    fn strict_palette_rejects_literal() {
        let src = "canvas 8 8\nstrict_palette true\npalette { A #000000 }\npixel 1 1 #ffffff\n";
        let doc = parser::parse(src).unwrap();
        let errs = validate::validate(doc).unwrap_err();
        assert_eq!(errs[0].code, "E013");
    }

    #[test]
    fn formatter_is_idempotent() {
        let doc = parser::parse(SLIME).unwrap();
        let once = fmt::format_document(&doc);
        let twice = fmt::format_document(&parser::parse(&once).unwrap());
        assert_eq!(once, twice);
    }
}
