# High Accuracy BNG to WGS84 demos #

Conversion between [EPSG:27700](http://spatialreference.org/ref/epsg/27700/) to [ESPG:4326](http://spatialreference.org/ref/epsg/4326/) is often completed without special attention to accuracy.  This can be due to lack of understanding of the British National Grid limitations or because the correction file size (~15MiB) is prohibitive.

This repository provides small demonstrations to achieve high quality transformations with minimal fuss.  

Includes examples using:

* [`ogr2ogr`](http://www.gdal.org/ogr2ogr.html) - single command + small config
* `C++` - user input is transformed and output with printf

