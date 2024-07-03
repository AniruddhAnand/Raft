#!/bin/bash
./kill.sh
(cargo run -- 5050)&
(cargo run -- 5051)&
(cargo run -- 5052)&
(cargo run -- 5053)&
(cargo run -- 5054)&
