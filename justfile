alias gv := grid-visualizer-3d

@_default:
    just --list

grid-visualizer-3d:
    cargo run --bin grid_visualizer_3d
