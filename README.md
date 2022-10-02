# Coupled-Explorers-LD51
game made in 48 hours for the Compo of Ludum Dare 51

* [ld submission](https://ldjam.com/events/ludum-dare/51/coupled-explorers)
* [itch page](https://noamzeise.itch.io/coupled-explorers)

## dependancies

* [sdl2](https://www.libsdl.org/) + [rust bindings](https://crates.io/crates/sdl2)
* [quick_xml](https://crates.io/crates/quick-xml)

## build

* get [rust and cargo](https://www.rust-lang.org/tools/install) if you dont have them
* install sdl2 + sld2_image + sdl2_mixer: [instructions](https://github.com/Rust-SDL2/rust-sdl2#sdl20-development-libraries) or use the .dlls included in the build on itch
* clone this repo
* run ```cargo run --release``` to build and run the game in release mode
