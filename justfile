alias gv := grid-visualizer-3d
alias bh := bullet-hell

@_default:
    just --list

grid-visualizer-3d:
    cargo run --bin grid_visualizer_3d

bullet-hell:
    cargo run --bin bullet-hell