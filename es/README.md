# Project Wormhole - ES
Library for parsing ES (ESM, ESP, ESL) files for Bethesda games  
Very bare-bones, not fully implemented

## Titles Currently Supported
- Fallout 4

## Future Supported Titles (in order)
- Starfield
- Fallout: New Vegas
- Fallout 3
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