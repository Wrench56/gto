// lib.rs - The core of the `.gto` Rust library

//! `gto-rs` - A high-performance parser for `.gto` (glTF Optimized) files.
//!
//! This library enables efficient loading of `.gto` files, supporting meshes,
//! materials, textures, animations, and skinning with SIMD acceleration.
//!
//! # Example Usage
//! ```
//! use gto_rs::GtoFile;
//! let gto = GtoFile::load("../../examples/model.gto").expect("Failed to load .gto file");
//! ```

use std::fs::File;
use std::io::{Read, Result, Seek, SeekFrom};

mod headers;

use crate::headers::{GtoAnimation, GtoHeader, GtoMaterial, GtoMesh, GtoNameTable, GtoTexture};

/// Represents a parsed `.gto` file
pub struct GtoFile {
    pub header: GtoHeader,
    pub meshes: Vec<GtoMesh>,
    pub materials: Vec<GtoMaterial>,
    pub textures: Vec<GtoTexture>,
    pub animations: Vec<GtoAnimation>,
    pub name_table: GtoNameTable,
}

impl GtoFile {
    /// Loads a `.gto` file from disk
    pub fn load(filename: &str) -> Result<Self> {
        let mut file = File::open(filename)?;
        let header = Self::read_header_from_file(&mut file)?;

        Ok(GtoFile {
            header,
            meshes: vec![],
            materials: vec![],
            textures: vec![],
            animations: vec![],
            name_table: GtoNameTable {
                mesh_names_offset: 0,
                material_names_offset: 0,
                animation_names_offset: 0,
                string_data_offset: 0,
            },
        })
    }

    /// Reads the `.gto` header from a file
    fn read_header_from_file<T: Read + Seek>(file: &mut T) -> Result<GtoHeader> {
        let mut buffer = [0u8; 32];
        file.seek(SeekFrom::Start(0))?;
        file.read_exact(&mut buffer)?;

        GtoFile::_get_header_from_buffer(buffer)
    }

    /// Reads the `.gto` header from a buffer
    pub fn read_header_from_data(buffer: Vec<u8>) -> Result<GtoHeader> {
        GtoFile::_get_header_from_buffer(buffer[..32].try_into().unwrap())
    }

    fn _get_header_from_buffer(buffer: [u8; 32]) -> Result<GtoHeader> {
        Ok(GtoHeader {
            magic: u32::from_le_bytes([buffer[0], buffer[1], buffer[2], buffer[3]]),
            version: u32::from_le_bytes([buffer[4], buffer[5], buffer[6], buffer[7]]),
            file_size: u64::from_le_bytes([
                buffer[8], buffer[9], buffer[10], buffer[11], buffer[12], buffer[13], buffer[14],
                buffer[15],
            ]),
            mesh_count: u32::from_le_bytes([buffer[16], buffer[17], buffer[18], buffer[19]]),
            material_count: u32::from_le_bytes([buffer[20], buffer[21], buffer[22], buffer[23]]),
            texture_count: u32::from_le_bytes([buffer[24], buffer[25], buffer[26], buffer[27]]),
            animation_count: u32::from_le_bytes([buffer[28], buffer[29], buffer[30], buffer[31]]),
            name_table_offset: 0,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_header_parsing() {
        let dummy_data: Vec<u8> = vec![
            0x46, 0x54, 0x4F, 0x47, // "GTOF" magic
            1, 0, 0, 0, // Version = 1
            100, 0, 0, 0, 0, 0, 0, 0, // File size = 100
            5, 0, 0, 0, // Mesh count = 5
            2, 0, 0, 0, // Material count = 2
            3, 0, 0, 0, // Texture count = 3
            4, 0, 0, 0, // Animation count = 4
        ];
        let header = GtoFile::read_header_from_data(dummy_data).unwrap();
        assert_eq!(header.magic, 0x474F5446);
        assert_eq!(header.version, 1);
        assert_eq!(header.file_size, 100);
        assert_eq!(header.mesh_count, 5);
        assert_eq!(header.material_count, 2);
        assert_eq!(header.texture_count, 3);
        assert_eq!(header.animation_count, 4);
    }
}
