#!/bin/bash

#INPUT_DIR=/Users/snodnipper/temp/bng-transformations/examples/docker/input
#OUTPUT_DIR=/Users/snodnipper/temp/bng-transformations/examples/docker/output

# Check environmental variables

if [ -z ${INPUT_DIR+x} ] ; then
  echo "ERROR: no INPUT_DIR environment variable"
  echo "Set the variable to an input directory containing 'input.vrt'"
  echo "i.e. \$INPUT_DIR=<empty_directory>"
  exit 1;
fi

if [ -z ${OUTPUT_DIR+x} ] ; then
  echo "ERROR: no OUTPUT_DIR environment variable"
  echo "Set the variable to an empty output directory, which will be created if necessary"
  echo "i.e. \$OUTPUT_DIR=<empty_directory>"
  exit 1;
fi

# Check input directory exists
if [ ! -d $INPUT_DIR ] ; then
    echo "Input directory doesn't exist: $INPUT_DIR"
    echo "Specify an input directory containing 'input.vrt'"
    echo "i.e. \$INPUT_DIR=<valid_directory>"
    exit 1;
fi

mkdir -p $OUTPUT_DIR

# clean output dir
rm -Rf $OUTPUT_DIR

# build docker container from scratch
docker build -t ogr2ogr .

# run docker container with volumes directed to env variables
docker run -it -v $INPUT_DIR:/root/input/ -v $OUTPUT_DIR:/root/output/ ogr2ogr
