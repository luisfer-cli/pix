pub const SPEC: &str = r#"pix DSL v0.1

Goal: deterministic pixel art as code. No antialiasing. Integer grid only.

Required:
  canvas WIDTH HEIGHT

Optional:
  background COLOR              # default transparent
  strict_palette true|false     # default false

Palette:
  palette {
    NAME COLOR
  }

Colors:
  transparent
  #RRGGBB
  #RRGGBBAA
  NAME from palette

Names:
  letters, digits, _, - ; must start with letter or _

Renderable statements, usable top-level or inside group:
  pixel X Y COLOR
  line X1 Y1 X2 Y2 COLOR
  rect X Y W H COLOR            # filled rectangle
  fill X Y COLOR                # flood fill current buffer region
  circle CX CY R COLOR          # filled circle
  group NAME at X Y {
    ...items...
  }

Animation:
  frame NAME {
    ...items...
  }

  animation NAME fps FPS {
    FRAME_NAME
    FRAME_NAME
  }

Coordinates inside groups are local. Moving a group moves all children.
Rendering order is textual. Pixels outside canvas are clipped.

Minimal example:
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

Implemented animation keywords: frame, animation.
Planned keywords: layer, component, use, cluster, mirror-x, mirror-y, sprite.
"#;

pub fn json_spec() -> serde_json::Value {
    serde_json::json!({
        "version": "0.1",
        "implemented": ["canvas", "background", "strict_palette", "palette", "group", "pixel", "line", "rect", "fill", "circle", "frame", "animation"],
        "colors": ["transparent", "#RRGGBB", "#RRGGBBAA", "palette name"],
        "principles": ["integer coordinates", "local group coordinates", "textual render order", "RGBA transparency", "no antialiasing"]
    })
}
