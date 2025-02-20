/// The `.gto` file header
#[repr(C)]
#[derive(Debug)]
pub struct GtoHeader {
    pub magic: u32, // "GTOF" Magic Identifier
    pub version: u32,
    pub file_size: u64,
    pub mesh_count: u32,
    pub material_count: u32,
    pub texture_count: u32,
    pub animation_count: u32,
    pub name_table_offset: u64,
}

/// Represents a mesh in the `.gto` file
#[derive(Debug)]
pub struct GtoMesh {
    pub name_offset: u64,
    pub vertex_count: u32,
    pub index_count: u32,
    pub material_index: u32,
}

/// Represents a material in the `.gto` file
#[derive(Debug)]
pub struct GtoMaterial {
    pub name_offset: u64,
    pub base_color_texture: u32,
    pub normal_map_texture: u32,
}

/// Represents a texture in the `.gto` file
#[derive(Debug)]
pub struct GtoTexture {
    pub name_offset: u64,
    pub width: u32,
    pub height: u32,
    pub data_offset: u64,
}

/// Represents an animation in the `.gto` file
#[derive(Debug)]
pub struct GtoAnimation {
    pub name_offset: u64,
    pub frame_count: u32,
    pub keyframe_offset: u64,
}

/// Represents the Name Table
#[derive(Debug)]
pub struct GtoNameTable {
    pub mesh_names_offset: u64,
    pub material_names_offset: u64,
    pub animation_names_offset: u64,
    pub string_data_offset: u64,
}
