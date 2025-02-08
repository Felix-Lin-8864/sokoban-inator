# sokoban-inator
Tutorial for the base game by [iolivia](https://github.com/iolivia) [here](https://sokoban.iolivia.me/c01-00-intro)!

## The Game
The classic sokoban game: push boxes onto spots!

### Rules
Win when all spots are covered by a box of a corresponding colour. Gray boxes and spots are special; they can match with any colour.

Every level, you get more players to control, and the possible number of boxes and spots that spawns increases! Please note that, if a box spawns on your players' positions, they get crushed, wallow, cry and disappear... However, this does make control easier (less clumsy) as you progress through the levels, so I guess its a sacrifice for the greater good?

### Extras
Also, you CAN squish your team by running into walls. This was definitely not a bug that I was too lazy to remove (actually wasn't; I fixed it to work regardless of player entity order on [lines 70 of input.rs](src/systems/input.rs) by using iteration-local Vecs, iter_moves). Thought it would be fun and more puzzle-like to have users try and figure out which players can be squished, but realised it was not idiomatic with many 2D multi-player control games.

Same with the multiple-player-spawning behaviour; initially, entity creation functions were called on the same hecs::World instance, causing new entities to be added to the current world, then fixed that bug by replacing the World instance with a new one. The multiple-player control seemed fun, so I implemented it correctly on lines [122-130 of map.rs](src/map.rs).

### Controls:
- Use arrow keys to move
- Press 'R' to restart the game

## The Project
Coded in Rust utilising the [ggez](https://docs.rs/ggez/latest/ggez/index.html) library for resource loading and event handling, and [hecs](https://docs.rs/hecs/latest/hecs/) library to implement an Entity-Component-System.

The base game was completed by following a [tutorial](https://sokoban.iolivia.me/c01-00-intro). My project builds on this with small tweaks - e.g. universal gray boxes/spots and multi-player control - and larger features - e.g. level-progression and random map generation.

