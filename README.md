# Project Wormhole
### All-in-one library for Bethesda Games

- `ESM` - Active Development
- `NIF` - Paused
- `BS2` - Paused


## ESM
- Working on data driven code generation for each record type
- `RawESM` - `#RRGGBB` WIP 
  - On demand data for editing
  - Indexes all top records in less than 300ms
  - Data is only converted to actual record structure on request
- `SmartESM` - WIP
  - Tries to load the data as a game would

## NIF
- Needs previous code to be copied over
- Currently parsing
  - Verticies - Done
  - Normals - Done
  - Skeleton - WIP 
    - Merging relies on esm to point to files
  - Animations - Not Started
  - Textures - WIP - Needs special post process for modern engines
- GLTF Export 
  - Can export everything that can be parsed
  - UE5 seems to struggle with skeleton export

## BA2
- Needs previous code to be copied over
- Base version fully working, just needs optimization