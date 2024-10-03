#!/usr/bin/env bash

cd "$( dirname "$0" )"

for dir in ../resources-launcher_*; do
	size=${dir##*_}
	echo $size
	inkscape --export-overwrite --export-filename="$dir/ic_launcher.png" -w ${size%x*} -h ${size#*x} icon.svg
done
