# Project Wormhole - ES
Library for parsing ES (ESM, ESP, ESL) files for Bethesda games  

## Supported Titles
- Starfield
- Fallout 4
- Fallout 3
- Fallout: New Vegas
- The Elder Scrolls V: Sentinel (hopefully)
- The Elder Scrolls V: Skyrim
- The Elder Scrolls IV: Oblivion

## Specification Overview

### Objects

#### Group
- A list of objects
- The header will hint at what the contents should be
- Can be the child of a record
- Most top level groups are record lists

#### Record
- A list of fields
- The header will change how the fields are read

#### Field
- Contains the actual data