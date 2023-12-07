#!/bin/bash

maxday=$(ls -d */ | sed -r 's|0*(.*)/|\1|g' | tail -1)
nextday=$((maxday + 1))
nextday_dir=$(printf "%02d" $nextday)
mkdir $nextday_dir
sed "s/X/$nextday/g" README.md.temp > $nextday_dir/README.md
