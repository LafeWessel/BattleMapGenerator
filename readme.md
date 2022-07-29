## Battle Map Generator for the "Game"

Takes a given position on the game board and the surrounding tiles to generate a battle map. Saves the map as a PNG and/or prints it to the console.

### Implementation

How should this be done?

- Take the battle tile and all tiles within 1 (7 total)
- Determine if there are roads/towns
- Determine how much should be forest, mountain, hill, plain, swamp, etc.
- Add tiles
    1. Rivers, using wave function collapse(?)
    1. Roads, using wave function collapse(?)
    1. Towns
    1. Mountains then hills
    1. Forests then swamps
    1. Default rest to plains
    1. Add bridges/fords
    1. Add Barricades
    1. Add other?
- Create and save image
- Print results