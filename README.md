# i3-sway-spiral

Change the i3/sway layout to splith/splitv depending on node size,
to open new windows in a spiral motion.

Inspired by XMonad and https://github.com/nwg-piotr/autotiling

![Spiral](i3-sway-spiral.png)

## Run

```
cargo run
```

You can get more verbose output using log levels, e.g.

```
# Using one of error, warn, info, debug, trace
RUST_LOG=info cargo run
```

## Build

```
cargo build --release
# binary is now in target/release/i3-sway-spiral
# run `strip` on the binary if you want to remove debug symbols
```

## License

i3-sway-spiral, open new windows in a spiral motion
Copyright (C) 2020 Riccardo Attilio Galli

This program is free software: you can redistribute it and/or modify
it under the terms of the GNU Affero General Public License as published by
the Free Software Foundation, either version 3 of the License, or
(at your option) any later version.

This program is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
GNU Affero General Public License for more details.

You should have received a copy of the GNU Affero General Public License
along with this program. If not, see <http://www.gnu.org/licenses/>.
