#!/bin/bash

if [ ! -f /usr/local/share/proj/OSTN02_NTv2.gsb ]; then
    echo "OSTN02 not available in /usr/local/share/proj/OSTN02_NTv2.gsb"
    exit
fi

if [ -f v2.csv ]; then
    rm v2.csv
fi

if [ -f bng_again.csv ]; then
    rm bng_again.csv
fi

if [ -f bng_again_numbers.csv ]; then
    rm bng_again_numbers.csv
fi

if [ -f input_numbers.csv ]; then
    rm input_numbers.csv
fi

echo "We are going to transform to V2"
echo 
echo "-----------------------------------------------------------------"
echo "| Using the ogr2ogr command we transform to V2.  Something like:"
echo "|"
echo "| ogr2ogr -dim 2 -f 'CSV' -t_srs '+proj=tmerc +lat_0=49 +lon_0=-2"
echo "|     +k=0.9996012717 +x_0=1400000 +y_0=900000 +ellps=WGS84"
echo "|     +datum=WGS84 +units=m +no_defs\"' \"v2.csv\" input.vrt"
echo "|     -lco GEOMETRY=AS_XY"
echo "|     -oo X_POSSIBLE_NAMES=GEOMETRY_X"
echo "|     -oo Y_POSSIBLE_NAMES=GEOMETRY_Y'"
echo "|"
echo "| Let us translate the example British National Grid CSV file to V2 (v2.csv)"
echo "|"
echo -n "Press [ENTER] to continue"
read
ogr2ogr -dim 2 -f 'CSV' -t_srs '+proj=tmerc +lat_0=49 +lon_0=-2 +k=0.9996012717 +x_0=1400000 +y_0=900000 +ellps=WGS84 +datum=WGS84 +units=m +no_defs"' "v2.csv" input.vrt -lco GEOMETRY=AS_XY 
echo "| We now have V2 values" 
echo "|"
echo "| We'll now: "
echo "| * rename X and Y columns to V2_X and V2_Y"
echo "| * round the V2 values to three decimal places"
echo "|"
echo -n "Press [ENTER] to continue"
read
cat v2.csv | awk 'BEGIN { FS = ","; OFS = FS ; } ;{ if (NR == 1) {$1="V2_X"; $2="V2_Y";} else {$1=sprintf("%.3f", $1); $2=sprintf("%.3f", $2)} print}' > v2.csv.swp
mv v2.csv.swp v2.csv
echo
echo "| Now we have a (rounded) V2 file, let us see if we convert it back to British National Grid"
echo "| "
echo "| We'll make a copy of the V2 file and then: "
echo "| * convert from V2 to BNG"
echo "| * round the BNG values to three decimal places"
echo
echo "| Ready to translate back to BNG values again?" 
echo "|"
echo -n "Press [ENTER] to continue"
read
ogr2ogr -dim 2 -f 'CSV' -t_srs '+proj=tmerc +lat_0=49 +lon_0=-2 +k=0.9996012717 +x_0=400000 +y_0=-100000 +ellps=airy +datum=OSGB36 +units=m +no_defs +nadgrids=/usr/local/share/proj/OSTN02_NTv2.gsb' "bng_again.csv" input_v2.vrt -lco GEOMETRY=AS_XY
cat bng_again.csv | awk 'BEGIN { FS = ","; OFS = FS ; } ;{ if (NR != 1) {$1=sprintf("%.3f", $1); $2=sprintf("%.3f", $2)} print}' > bng_again.csv.swp
mv bng_again.csv.swp bng_again.csv

echo
echo "| OK, so we have translated from BNG to V2 and now back to BNG"
echo "| Let us compare before and after"
cat input.csv | awk 'BEGIN { FS = ","; } ;{ if (NR != 1) { print $9 FS $10 } }' > input_numbers.csv
cat bng_again.csv | awk 'BEGIN { FS = ","; OFS = FS ; } ;{ if (NR != 1) { print $1 FS $2 } }' > bng_again_numbers.csv
diff input_numbers.csv bng_again_numbers.csv

if [ $? -eq 0 ]; then
    echo "| 👍 - Perfect Transformation Sir!"
else
    echo "| ❌ - FAIL!  Compare input_numbers.csv against the transformed bng_again_numbers.csv values!"
fi