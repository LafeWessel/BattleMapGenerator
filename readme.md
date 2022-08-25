## Battle Map Generator for the "Game"

Takes a given position on the game board and the surrounding tiles to generate a battle map. Saves the map as a PNG and/or prints it to the console.

### Implementation

How should this be done?

- Take the attacker, defender, and 2 adjacent tiles (4 total)
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

### Thoughts

- Need to determine which input tiles have roads to determine which edges should have exit roads
- Need to determine whic input tiles have rivers to determine which edges should have exit rivers
- Should take proximity to nearest river to determine if any streams should be present
- Need to calculate how hilly/mountainous the terrain should be
- Need to figure out how much a certain owned section should be like its campaign tile

### Notes


- Formula for finding total hexes within radius of a hex is
 ``` math
 3r^2 + 3r + 1
 ```