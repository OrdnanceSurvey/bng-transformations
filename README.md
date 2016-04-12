# High Accuracy BNG to WGS84 demos #

Conversion between [EPSG:27700](http://spatialreference.org/ref/epsg/27700/) to [ESPG:4326](http://spatialreference.org/ref/epsg/4326/) is often completed without special attention to accuracy.  This can be due to lack of understanding of the British National Grid limitations or because the correction file size (~15MiB) is prohibitive.

This repository provides small demonstrations to achieve high quality transformations with minimal fuss.  

Includes examples using:

* `docker` - a docker container to transform your CSV data using ogr2ogr
* [`ogr2ogr`](http://www.gdal.org/ogr2ogr.html) - single command + small config
* `C++` - user input is transformed and output with printf

# OSTN02 #
The British National Grid (BNG) is not a perfect grid because it was made prior to modern high accuracy technology.  It can be likened to a grid made from string, where certain parts are slightly pulled to signify inaccuracy.

In its original form OSTN02 consists of a plane 700km by 1250km grid of 1km resolution with an eastings shift and northing shift at each grid node.  The OSTNS02 file corrects the funamental BNG inaccuracies, which enables coordinate values to be transformed to other projections (e.g. EPSG:4326) via mathematical function whilst maintaining accuracy to the true location.

Further details are [available](https://www.ordnancesurvey.co.uk/business-and-government/help-and-support/navigation-technology/os-net/ostn02-ntv2-format.html).
