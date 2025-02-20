# The .gto file spec

The `.gto` (glTF Optimized) format is a fully binary, optionally SIMD-accelerated, and direct-access alternative to `.glTF`/`.glb`, designed for fast and easy parsing and GPU-ready data.

## File layout

- Header
- Mesh Table
- Material Table
- Texture Table
- Sampler Table
- Light Table
- Node Table
- Animation Table
- Skin Table
- Accessor Table
- Morph Target Table
- Sparse Accessor Table
- Binary Data Blocks

