#!/bin/bash

LANG=$1
YEAR=$2

cd `git rev-parse --show-toplevel`

if [[ -z $LANG ]]; then
	echo "Please choose language template. Available options are:"
	ls templates/ | grep -v setup
	exit 1
fi

if [[ ! -e templates/$LANG ]]; then
	echo "'$LANG' not available in templates"
	exit 1
fi

if [[ -z $YEAR ]]; then
	YEAR=$(date '+%Y')
	echo "Year not specified using current: $YEAR"
fi

if [[ -e $YEAR ]]; then
	echo "$YEAR already exists!"
	exit 1
fi

mkdir -p $YEAR/$LANG/
cp -r templates/$LANG/year/* $YEAR/$LANG/
ln -s ../../templates/$LANG/day $YEAR/$LANG/daily-template 

