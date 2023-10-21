#!/bin/sh
cargo watch -w live -x 'build -p live'
#cargo watch -w live -- ./recomp.sh
#cargo watch -w live -- script -c "cargo build -p live" compout
