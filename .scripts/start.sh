#!/bin/bash
cd /home/ec2-user/hodler-signal/
~/.cargo/bin/cargo b -r
sudo pkill signal
RUST_LOG=debug ~/.cargo/bin/cargo r -r </dev/null &>/var/log/hodler-signal &
