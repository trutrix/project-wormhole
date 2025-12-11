# Project Wormhole
### All-in-one library for Bethesda Games

- `ESM` - Active
- `NIF` - Paused - Need ESM to advance
- `BA2` - Paused - Need ESM to advance
- `BGSM` - Not Started
- `SWF` - Not Started


## ESM
- Working on data driven code generation for each record type
- `RawESM` - `WIP`
  - On demand data for faster editor performance
  - Indexes all top records in less than 300ms
  - Data is only converted to actual record structure on request
- `ESMFull` - `WIP`
  - Parse all data, take advantage of multi-threading
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


## CLI
- Dumper - `Operational WIP`
  - Dumps all top level field idens and the sizes present in the file
  - Should help identify fixed structs
- Heightmap Extractor - `WIP`
  - I was unsatisfied how long it took creation kit to do this
  - I have achieved this 2 times before, just didn't save the code `:(`

## BGSM
  - Material files parser `Not Started`


## GUI Application
  - Rust has extremely young UI frameworks that are poorly documented
  - May be easier to export libs to different langs
  - Tauri is the frontrunner for rust but requires an enormous amount of effort to get the features I want

# Sources
- Fallout 4 & Creation Kit
- [FO4Edit](https://tes5edit.github.io/) - Best tool there is
- [UESP](https://en.uesp.net/wiki/Skyrim_Mod:Mod_File_Format) - Extremely helpful understanding records and groups
- [NifSkope](https://github.com/niftools/nifskope) - Has almost all info regarding nif files, seems unmaintained right now
- Places I didn't write down (I promise I'll find you again)


# Project Notes
- Need to switch some types to use standard libs that perform much better (ie `glam` for vecs and matrices)
- `nom_derive` works thusfar but seems poorly suited for versioned files, if we ever want to support all the games this must be addressed
- I want to implement `zerocopy` at some point for transmutations
- Unfortunately I did not account for serializing data back down, making a lot of this project unusable for editing
