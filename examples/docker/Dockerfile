FROM ubuntu:xenial
MAINTAINER Jake Skeates <jskeates@me.com>

RUN apt-get -y update \
 && apt-get -y upgrade \
 && apt-get -y install gdal-bin \
 && apt-get clean \
 && rm -rf /var/lib/apt/lists/* /tmp/* /var/tmp/* \
 && mkdir -p /usr/local/share/proj \
 && mkdir -p /root/ogr2ogr

ADD OSTN02_NT* /usr/local/share/proj/

WORKDIR /root/input
CMD ogr2ogr -f 'CSV' -lco GEOMETRY=AS_XYZ -t_srs EPSG:4326 -nln output /root/output/output.csv input.vrt
