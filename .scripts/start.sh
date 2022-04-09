#!/bin/bash
cd /home/ec2-user/hodler-signal/
~/.cargo/bin/cargo b
RUST_LOG=debug ~/.cargo/bin/cargo r </dev/null &>/var/log/hodler-signal &
