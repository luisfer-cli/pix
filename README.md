# pix

Deterministic pixel-art renderer for a small `.pix` DSL. It renders pixel-perfect PNGs and animated GIFs with integer coordinates, palette validation, frames, animations, and spritesheet export.

## Features

- `.pix` text DSL for pixel art as code
- Deterministic rendering, no antialiasing
- RGBA colors and named palettes
- Groups with local coordinates
- Validation, formatting, AST inspection, and spec output
- Animation frames, animated GIFs, and spritesheet metadata JSON

## Install

```bash
cargo install --path .
```

Or run locally:

```bash
cargo run -- <command>
```

## Quick example

```pix
canvas 16 16
background transparent
strict_palette true

palette {
    O #181425
    G #3fa34d
    L #7bd88f
}

frame idle-1 {
    group slime at 3 5 {
        rect 0 1 10 5 G
        pixel 2 2 O
        pixel 7 2 O
        circle 5 3 2 L
    }
}

frame idle-2 {
    group slime at 3 6 {
        rect 0 1 10 4 G
        pixel 2 2 O
        pixel 7 2 O
        circle 5 2 2 L
    }
}

animation idle fps 6 {
    idle-1
    idle-2
}
```

## Usage

Validate a file:

```bash
pix check examples/slime_anim.pix
```

Render a static `.pix` file:

```bash
pix render examples/slime.pix -o output/slime.png
```

Render a single frame:

```bash
pix render examples/slime_anim.pix --frame idle-1 -o output/slime-idle-1.png
```

Export animation frames as PNG files:

```bash
pix anim examples/slime_anim.pix -a idle --out-dir output/slime-idle
```

Export an animated GIF:

```bash
pix gif examples/slime_anim.pix -a idle -o output/slime-idle.gif
```

Export a spritesheet and metadata:

```bash
pix sheet examples/slime_anim.pix -a idle \
  -o output/slime-idle-sheet.png \
  --json output/slime-idle-sheet.json \
  --columns 2
```

Format a file:

```bash
pix fmt examples/slime_anim.pix --write
```

Print the DSL spec:

```bash
pix spec
```

## License

MIT
