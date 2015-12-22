#include <iostream>
#include <PROJ/proj_api.h>

int bng_1m_accuracy() {
    // e.g. x = -1.8, y = 51.18;  => Easting: 414075.69   Northing: 142326.96
    
    projPJ pj_src, pj_dst;
    double x, y;
    int p;
    
    // EPSG:4326 definition: http://spatialreference.org/ref/epsg/4326/proj4/
    const char* src = "+proj=longlat +ellps=WGS84 +datum=WGS84 +no_defs";
    
    // EPSG:27700 definition: http://spatialreference.org/ref/epsg/27700/proj4/
    // NTv2 correction data: https://www.ordnancesurvey.co.uk/business-and-government/help-and-support/navigation-technology/os-net/ostn02-ntv2-format.html
    const char* dst = "+proj=tmerc +lat_0=49 +lon_0=-2 +k=0.9996012717 +x_0=400000 +y_0=-100000 +ellps=airy +datum=OSGB36 +units=m +no_defs +nadgrids=/usr/local/share/proj/OSTN02_NTv2.gsb";
    
    if (!(pj_src = pj_init_plus(src)) )
        exit(1);
    if (!(pj_dst = pj_init_plus(dst)) )
        exit(1);
    while (scanf("%lf %lf", &x, &y) == 2) {
        x *= DEG_TO_RAD;
        y *= DEG_TO_RAD;
        p = pj_transform(pj_src, pj_dst, 1, 1, &x, &y, NULL );
        printf("%.2f\t%.2f\n", x, y);
    }
    exit(0);
}

int bng_2m_accuracy() {
    // e.g. x = -1.8, y = 51.18;  => Easting: 414076.45 Northing: 142326.42
    
    projPJ pj_src, pj_dst;
    double x, y;
    int p;
    
    // EPSG:4326 definition: http://spatialreference.org/ref/epsg/4326/proj4/
    const char* src = "+proj=longlat +ellps=WGS84 +datum=WGS84 +no_defs";
    
    // EPSG:27700 definition: http://spatialreference.org/ref/epsg/27700/proj4/
    const char* dst = "+proj=tmerc +lat_0=49 +lon_0=-2 +k=0.9996012717 +x_0=400000 +y_0=-100000 +ellps=airy +datum=OSGB36 +units=m +no_defs";
    
    if (!(pj_src = pj_init_plus(src)) )
        exit(1);
    if (!(pj_dst = pj_init_plus(dst)) )
        exit(1);
    while (scanf("%lf %lf", &x, &y) == 2) {
        x *= DEG_TO_RAD;
        y *= DEG_TO_RAD;
        p = pj_transform(pj_src, pj_dst, 1, 1, &x, &y, NULL );
        printf("%.2f\t%.2f\n", x, y);
    }
    exit(0);
}

int main(int argc, const char * argv[]) {
    // e.g. Stonehenge: -1.8 51.18
    //defaultSimple();
    //bng_2m_accuracy();   // 414076.45 142326.42
    bng_1m_accuracy();   // 414075.69 142326.96
    return 0;
}

