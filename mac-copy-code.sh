#!/bin/bash

cd "${0%/*}"
./target/release/boilerkey-rs | tee /dev/fd/1 | tail -1 | pbcopy
terminal-notifier -message "Copied login code to clipboard"
