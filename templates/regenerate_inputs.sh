#!/bin/bash

set -e

LANG=$1

if [[ -z $LANG ]]; then
	echo "Please set language."
	exit 1
fi

for year in $(find adventofcode-inputs/ -maxdepth 1 -type d ! -path adventofcode-inputs/ -exec basename {} \;); do
	for day in $(ls adventofcode-inputs/$year); do
		ln -s ../../../adventofcode-inputs/$year/$day/input1.txt  $year/$LANG/$day/input1.txt
		ln -s ../../../adventofcode-inputs/$year/$day/input2.txt  $year/$LANG/$day/input2.txt
	done
done

