# tl;dr #
High accuracy EPSG:27700 to ESPG:4326 transformation can be achieved using the [OSTN15 correction file](https://www.ordnancesurvey.co.uk/docs/gps/OSTN15_NTv2.zip).

[`ogr2ogr`](http://www.gdal.org/ogr2ogr.html) can use the correction file to achieve ~1m accuracy.

1. update the geometry X, Y values in `input.vrt` and ensure that '+nadgrids' points to the downloaded and extracted OSTN15 file.

2. execute `ogr2ogr -f 'CSV' -lco GEOMETRY=AS_XYZ -t_srs EPSG:4326 -nln output output.csv input.vrt` 

Note: OSTN15 _should be used_ in all applications where transformation accuracy is important.  Formulaic transformations only achieve ~2m accuracy. 

# Transform #
[`ogr2ogr`](http://www.gdal.org/ogr2ogr.html) is a command provided by [GDAL](http://www.gdal.org/).  It enables one to convert geographic file formats and specify any projection transformations.  We are therefore able to convert source British National Grid (BNG) / [EPSG:27700](http://spatialreference.org/ref/epsg/27700/) data to a global spatial reference such as [EPSG:4326](http://spatialreference.org/ref/epsg/4326/), which is used by the GPS satellite navigation system and for NATO military geodetic surveying.

In addition to executing the commands, below, it is necessary to configure the `input.vrt` configuration file.  Within the file it is necessary to specify the input coordinate fields and the location of the OSTN15 correction file.

Within the example input.vrt file, one can change:
* `GEOMETRY_X` / `GEOMETRY_Y` to be the EASTING and NORTHING column names according to the local dataset
* `+nadgrids=` to point to the OSTN15 file location, which is included in this git repository for your convenience.

## Shapefile output ##
`ogr2ogr -t_srs EPSG:4326 output.shp input.vrt`

## CSV output ##
`ogr2ogr -f 'CSV' -lco GEOMETRY=AS_XYZ -t_srs EPSG:4326 -nln output output.csv input.vrt`

Note: the _-nln_ flag assigns an alternate name to the new layer, which avoids output problems.

## Performance ##
My Macbook Pro transformed ~3 million lines in ~2 minutes.

```
macbookpro:cmd snodnipper$ date;ogr2ogr -f 'CSV' -lco GEOMETRY=AS_XYZ -t_srs EPSG:4326 -nln output output.csv input.vrt;date
Tue 22 Dec 2015 12:28:00 GMT
Tue 22 Dec 2015 12:30:20 GMT
macbookpro:cmd snodnipper$ wc -l output.csv 
 2820553 output.csv
```

