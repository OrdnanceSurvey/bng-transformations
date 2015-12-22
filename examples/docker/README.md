# BNG to WGS84 transformation Docker container #

This is a simple docker container that includes ogr2ogr and the OSTN02 correction
file.

You can specify the directory containing your British National Grid CSV file
and configure the input.vrt file for your specific requirements.

Transforming your CSV is as simple as:

```
export INPUT_DIR=[source directory containing input.vrt]
export OUTPUT_DIR=[directory to add transformed csv]
./run.sh
```

Upon completion you should see an `output.csv` file in your output directory.

Note: the input directory contains example files.

