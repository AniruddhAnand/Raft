#!/bin/bash
./kill.sh
(cargo run -- 5050)&
(cargo run -- 5051)&
(sleep 5; cargo run -- 5052)&
(sleep 10; cargo run -- 5053)&
(cargo run -- 5054)&
