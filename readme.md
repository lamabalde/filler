# filler

[Project description](https://github.com/01-edu/public/tree/master/subjects/filler)

Solved in grit:lab in July 2024

### Requirements to run:
* [Docker](https://docs.docker.com/get-docker/)
* [Rust](https://www.rust-lang.org/tools/install)


## run
`cd solution`
`cargo build`

Choose your desired settings, and run:
`./start.sh`
# Filler docker image

- To build the image `docker build -t filler .`
- To run the container `docker run -v "$(pwd)/solution":/filler/solution -it filler`. This instruction will open a terminal in the container, the directory `solution` will be mounted in the container as well.
- Example of a command in the container `./linux_game_engine -f maps/map01 -p1 linux_robots/bender -p2 linux_robots/terminator`
- Your solution should be inside the `solution` directory so it will be mounted and compiled inside the container and it will be able to be run in the game engine.

## Notes

- `Terminator` is a very strong robot so it's optional to beat him.
- For M1 Macs use `m1_robots` and `m1_game_engine`.
`./linux_game_engine -f maps/map00 -p1 linux_robots/bender -p2 solution/target/debug/solution`
`./linux_game_engine -f maps/map00 -p1 solution/target/debug/solution -p2 linux_robots/bender  `

## Authors:
* [mabalde](https://learn.zone01dakar.sn/git/mabalde)

