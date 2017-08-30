#!/bin/sh

# Generates a new line for today's date at the end of the file 'loc.txt'.
# Only Rust is counted at the moment.
loc ./qork/src | grep 'Rust' | awk -v T="%Y-%m-%d " '{ print strftime(T) $0 }' >> ./loc.txt


