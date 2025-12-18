use std::path::PathBuf;

use serde::ser::SerializeStruct;

use crate::header::BA2Header;

use super::dev::*;



pub struct BA2Archive {
    pub header: BA2Header,
    pub files: BTreeMap<String, BA2Entry>,
    file_handle: File
}

impl BA2Archive {
    pub fn open(path: &str) -> Result<Self, std::io::Error> {

        // Open file
        let mut file_handle = File::open(path)?;

        // Read header
        let mut buf = [0u8; 24];
        file_handle.read_exact(&mut buf)?;

        // Parse header
        let (_, header) = BA2Header::parse(&buf).unwrap();

        // Initialize file map
        let mut files = BTreeMap::new();

        // Read name table
        let mut names = get_file_names(&mut file_handle, header.name_table_offset)?;

        // Seek to start of file entries
        file_handle.seek(SeekFrom::Start(24))?;

        // Change behavior based on archive type
        if header.is_general_archive() {

            // General archives have a name table and a list of file entries, no nested structure
            // Create buffer for file entries (36 bytes each)
            let mut buf = vec![0u8; (header.file_count*36) as usize];

            // Read file entries into buffer
            file_handle.read_exact(&mut buf)?;

            // Parse general file entries
            let (_, mut items) = many0(complete(GeneralEntry::parse))(&buf).unwrap();
            
            // Check if name table and entry count match
            if names.len() != items.len() {
                return Err(std::io::Error::new(std::io::ErrorKind::InvalidData, "Name table and entry count mismatch"));
            }

            // Create file map from names and entries
            for _i in 0..names.len() {
                files.insert(standardize_path(names.pop().unwrap().as_str()), items.pop().unwrap().into());
            }

            } else if header.is_texture_archive() {

            // Texture archives have a name table and a list of texture entries, each with a list of chunks, allocating the correct memory beforehand is not possible
            

            // Create buffer for texture entries (24 bytes each)
            let mut entries = Vec::new();

            // Read texture entries into buffer
            for _i in 0..header.file_count {

                // Read texture entry
                file_handle.read_exact(&mut buf)?;

                // Parse texture entry
                let (_, entry) = TextureEntry::parse(&buf).unwrap();

                // Create buffer for chunks
                let mut chunk_buf = vec![0u8; entry.num_chunks as usize * entry.chunk_header_size as usize];

                // Read chunks into buffer
                file_handle.read_exact(&mut chunk_buf)?;
                
                // Parse chunks
                let (_, chunks) = count(TextureChunk::parse, entry.num_chunks as usize)(&mut chunk_buf).unwrap();

                // Store entry and chunks
                entries.push((entry, chunks));

            }

                // Combine iterator of names and entries into a single iterator
            for (name, entry) in names.iter().zip(entries.iter()) {

                // Get the chunk with the highest mip level
                let max_lod = entry.1[0];

                // Create file map from names and entries
                files.insert(standardize_path(name), BA2Entry {
                    packed_size: max_lod.packed_size,
                    unpacked_size: max_lod.unpacked_size,
                    offset: max_lod.offset,
                    texture: Some(entry.0.clone())
                });
            }
        } else {
            return Err(std::io::Error::new(std::io::ErrorKind::InvalidData, "Unknown archive type"));
        }

        Ok(BA2Archive { header, files, file_handle })

    }

    pub fn read_file(&mut self, name: &str) -> Result<Vec<u8>, std::io::Error> {
        // Standardize path
        let name = standardize_path(name);

        // Get file entry
        let entry = self.files.get(name.as_str()).ok_or(std::io::Error::new(std::io::ErrorKind::NotFound, "File not found"))?;
        
        // Create buffer for file, size depends on if it's packed or not
        let mut raw_buf = if entry.packed_size > 0 {
            vec![0u8; entry.packed_size as usize]
        } else {
            vec![0u8; entry.unpacked_size as usize]
        };

        // Seek to file positions
        self.file_handle.seek(SeekFrom::Start(entry.offset))?;

        // Read file into memory
        self.file_handle.read_exact(&mut raw_buf)?;

        let real_buf;
        
        // If packed, decompress
        if entry.packed_size > 0 {
            let mut dec = flate2::read::ZlibDecoder::new(raw_buf.as_slice());
            let mut buf = vec![0u8; entry.unpacked_size as usize];
            dec.read_exact(&mut buf)?;
            real_buf = buf;
        } else {
            real_buf = raw_buf;
        }

        if self.header.is_texture_archive() && name.ends_with(".dds") {

            let td = entry.texture.as_ref().unwrap();

            let mut dds = image_dds::ddsfile::Dds::new_dxgi(image_dds::ddsfile::NewDxgiParams {
                height: td.texture_height as u32,
                width: td.texture_width as u32,
                depth: None,
                format: format_from_u8(td.format),
                mipmap_levels: Some(1),
                array_layers: None,
                caps2: None,
                is_cubemap: td.is_cubemap == 1,
                resource_dimension: image_dds::ddsfile::D3D10ResourceDimension::Texture2D,
                alpha_mode: image_dds::ddsfile::AlphaMode::Opaque,
            }).unwrap();

            dds.data = real_buf;
            let mut dds_raw = Vec::new();
            dds.write(&mut dds_raw).unwrap();

            Ok(dds_raw)

        } else {
            Ok(real_buf)
        }

    }

    pub fn read_all_files(&mut self) -> Vec<(String, Vec<u8>)> {
        let mut out_files: Vec<(String, Vec<u8>)> = Vec::new();

        let file_copy = self.files.clone();
        let keys: Vec<&String> = file_copy.keys().collect();

        for name in keys {
            let data = self.read_file(name).unwrap();
            out_files.push((name.clone(), data));
        }

        out_files
    }
}

impl std::fmt::Debug for BA2Archive {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "BA2 Archive\nType: {:?}\nFiles: {}", self.header.archive_type, self.files.len())
    }
}




#[derive(Debug, Clone)]
pub struct BA2Entry {
    pub packed_size: u32,
    pub unpacked_size: u32,
    pub offset: u64,
    pub texture: Option<TextureEntry>
}


#[derive(Debug, NomLE, Clone)]
pub struct GeneralEntry {
    pub unknown0: u32,
    pub ext: [u8;4],
    pub unknown1: u32,
    pub unknown2: u32,
    pub offset: u64,
    pub packed_size: u32,
    pub unpacked_size: u32,
    pub unknown3: u32
}


impl From<GeneralEntry> for BA2Entry {
    fn from(value: GeneralEntry) -> Self {
        BA2Entry {
            packed_size: value.packed_size,
            unpacked_size: value.unpacked_size,
            offset: value.offset,
            texture: None
        }
    }
}


#[derive(Debug, NomLE, Clone)]
pub struct TextureEntry {
    pub filename_hash: u32,
    pub file_extension: u32,
    pub directory_hash: u32,
    pub unknown: u8,
    pub num_chunks: u8,
    pub chunk_header_size: u16,
    pub texture_height: u16,
    pub texture_width: u16,
    pub mip_levels: u8,
    pub format: u8,
    pub is_cubemap: u8,
    pub tile_mode: u8
}

#[derive(Debug, NomLE, Clone, Copy)]
pub struct TextureChunk {
    pub offset: u64,
    pub packed_size: u32,
    pub unpacked_size: u32,
    pub start_mip: u16,
    pub end_mip: u16,
    pub align: u32
}

#[derive(Debug, NomLE)]
pub struct TextureData {
    pub chunk: TextureChunk,
    pub data: Vec<u8>
}



pub struct BA2ArchiveGroup {
    pub archives: Vec<BA2Archive>
}

impl BA2ArchiveGroup {

    pub fn open_all(dir: PathBuf) -> Result<Self, String> {
        let mut archives = Vec::new();

        for entry in std::fs::read_dir(dir).unwrap() {
            let path = entry.unwrap().path();
            let path_str = path.to_str().unwrap();

            if path_str.ends_with(".ba2") {
                match BA2Archive::open(path_str) {
                    Ok(archive) => archives.push(archive),
                    Err(e) => return Err(format!("Error opening archive: {}", e))
                }
            }
        }

        Ok(BA2ArchiveGroup { archives })
    }

    pub fn read_file(&mut self, name: &str) -> Result<Vec<u8>, std::io::Error> {
        for archive in self.archives.iter_mut() {
            if let Ok(data) = archive.read_file(name) {
                return Ok(data);
            }
        }

        Err(std::io::Error::new(std::io::ErrorKind::NotFound, "File not found"))
    }

}


// ================================================================================

pub struct BA2Archive2 {
    pub header: BA2Header

}

impl Parse<&[u8]> for BA2Archive2 {
    fn parse(i: &[u8]) -> IResult<&[u8], Self, error::Error<&[u8]>> {
        let (i, header) = BA2Header::parse(i)?;

        Ok((i, BA2Archive2 { header }))
    }
}

impl serde::Serialize for BA2Archive2 {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut state = serializer.serialize_struct("BA2Archive2", 1)?;
        state.serialize_field("header", &self.header)?;
        state.end()
    }
}