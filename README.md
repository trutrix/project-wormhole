# Project Wormhole
### All-in-one library for Bethesda Games

- `ESM` - Active
- `NIF` - Paused - Need ESM to advance
- `BS2` - Paused - Need ESM to advance


## ESM
- Working on data driven code generation for each record type
- `RawESM` - `WIP`
  - On demand data for editing
  - Indexes all top records in less than 300ms
  - Data is only converted to actual record structure on request
- `SmartESM` - `WIP`
  - Tries to load the data as a game would, prioritizing certain data

## NIF
- Needs previous code to be migrated over
- `Current progress:`
  - `Verticies` - `Done`
  - `Normals` - `Done`
  - `UVs` - `Done`
  - `Skeleton` - `WIP` 
    - Merging relies on esm to point to files
  - `Animations` - `Not Started`
  - `Textures` - `WIP` - Needs special post process for modern engines
- `GLTF Export `
  - Can export everything that can be parsed
  - UE5 seems to struggle with importing advanced parts of GLTF skeletons, something about multiple weights

## BA2
- Needs previous code to be migrated over
- Base version fully working, just needs optimization