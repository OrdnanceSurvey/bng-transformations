extern crate proj;
use proj::Proj;

extern crate geo_types;
use geo_types::Point;
use std::vec::Vec;

const BNGV2: &'static str =
    "+proj=tmerc +lat_0=49 +lon_0=-2 +k=0.9996012717 +x_0=1400000 +y_0=900000 +ellps=WGS84 +datum=WGS84 +units=m +no_defs -f \"%.3f\"";

const BNGV1_OSTN15: &'static str =
    "+proj=tmerc +lat_0=49 +lon_0=-2 +k=0.9996012717 +x_0=400000 +y_0=-100000 +ellps=airy +datum=OSGB36 +units=m +no_defs -f \"%.3f\" +nadgrids=/usr/local/share/proj/OSTN15_NTv2_OSGBtoETRS.gsb";

use std::io;
use std::fmt;

extern crate regex;

#[derive(Debug, Copy, Clone)]
pub struct LocationSet {
    bng: Point<f64>,
    bng_v2: Point<f64>,
    etrs89: Point<f64>,
}

impl fmt::Display for LocationSet {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({:?}, {:?})", self.bng.x(), self.bng.y())
    }
}

fn main() {
    println!("#########################################################");
    println!("           Experimental V2 Location Transformer          ");
    println!("                                                         ");
    println!("                   Ordnance Survey 2018                  ");
    println!("                                                         ");
    println!("                             ___    ___ ___    ___       ");
    println!("  OS Data Office            |\\  \\  /  /|\\  \\  /  /|  ");
    println!("  Data Science Group         \\ \\  \\/  /|\\\\  \\/  / /");
    println!("                              \\ \\    / / \\\\    / /   ");
    println!("  Input:                       /     \\/   \\\\  / /     ");
    println!("    > OSTN15                  /  /\\   \\  __/ / /       ");
    println!("    > V2                     /__/ /\\ __||\\__/ /        ");
    println!("                            |__|/ \\|___||___|/          ");
    println!("                                                         ");
    println!("#########################################################");
    println!("# Helping you through the transformation!               #");
    println!("#########################################################");
    println!();

    // warning: this will crash if the coordinates are out of the range for the grid!
    let mut bng = Proj::new(BNGV1_OSTN15).unwrap();
    let mut bng_v2 = Proj::new(BNGV2).unwrap();

    let bool_normal = true;

    let mut input_type = InputType::UNSPECIFIED;

    let mut x = 0.0 as f64;
    let mut y = 0.0 as f64;

    loop {

        if bool_normal {
            // get input type
            match input_type {
                InputType::UNSPECIFIED => {
                    input_type = request_input_coordinate_type();
                }
                _ => {}
            }

            // special handling for test
            match input_type {
                InputType::TEST => {
                    test(&mut bng, &mut bng_v2);
                    input_type = InputType::UNSPECIFIED;
                    continue;
                }
                _ => {}
            }

            match request_input_coordinates(&input_type) {
                Some(point) => {
                    x = point.x();
                    y = point.y();
                }
                None => {
                    input_type = InputType::UNSPECIFIED;
                    continue;
                }
            }
        } else {
            input_type = InputType::BNG;
            x = 1393.0196;
            y = 13494.9764;
        }

        println!("Using x: {:?} y: {:?}", x, y);

        match input_type {
            InputType::BNG => {
                match get_location_set_for_bng(x, y, &mut bng, &mut bng_v2) {
                    Ok(location_set) => {
                        //println!("👍👍👍👍👍👍👍 🌍🌍🌍");
                        print_locationset(location_set);
                    },
                    Err(error) => {
                        match error as BngError {
                            BngError::LocationSetError(ls) => {
                                println!("❌❌❌❌❌❌❌ {}", ls);
                                panic!("There was a problem!");
                            }
                        }
                    },
                }
            }
            InputType::BNGv2 => {

                match get_location_set_for_bngv2(x, y, &mut bng, &mut bng_v2) {
                    Ok(location_set) => {
                        //println!("👍👍👍👍👍👍👍 🌍🌍🌍");
                        print_locationset(location_set);
                    },
                    Err(error) => {
                        match error as BngError {
                            BngError::LocationSetError(ls) => {
                                println!("❌❌❌❌❌❌❌ {}", ls);
                                panic!("There was a problem!");
                            }
                        }
                    },
                }
            }
            InputType::ETRS89 => {
                println!("oioioi lon:{:?}, lat:{:?}", x, y);
                match get_location_set_for_etrs(y, x, &mut bng, &mut bng_v2) {
                    Ok(location_set) => {
                        print_locationset(location_set);
                    },
                    Err(error) => {
                        match error as BngError {
                            BngError::LocationSetError(ls) => {
                                println!("❌❌❌❌❌❌❌ {}", ls);
                                panic!("There was a problem!");
                            }
                        }
                    },
                }
            }
            _ => {} // IGNORE ETRS89
        }
    }
}

fn test(bng: &mut Proj, bng_v2: &mut Proj) {
    // BNGv1
    const MIN_X_F64: f64 = 140324.0;
    const MAX_X_F64: f64 = 411264.0;
    const MIN_Y_F64: f64 = 17320.0;
    const MAX_Y_F64: f64 = 1230275.0454;

    const ITERATIONS: i32 = 100000000;
    println!("I'm gonna do {:?} moves", ITERATIONS);

    const X_INCREMENT: f64 = (MAX_X_F64 - MIN_X_F64) / ITERATIONS as f64;
    const Y_INCREMENT: f64 = (MAX_Y_F64 - MIN_Y_F64) / ITERATIONS as f64;

    let mut start_x = as3dp(MIN_X_F64);
    let mut start_y = as3dp(MIN_Y_F64);

    // failures
    let mut failures = Vec::new();

    for _x in 0..ITERATIONS {
        match get_location_set_for_bng(start_x, start_y, bng, bng_v2) {
            Ok(_ls) => {
                // let location_set = _ls;
                //vec.push(location_set);
                //println!("👍👍👍👍👍👍👍 🌍🌍🌍");
                //print_locationset(location_set);
            },
            Err(error) => {
                match error as BngError {
                    BngError::LocationSetError(location_set) => {
                        //println!("❌❌❌❌❌❌❌ {}", location_set);
                        failures.push(location_set);
                        panic!("There was a problem Jeff!");
                    }
                }
            },
        }

        start_x = as3dp(start_x + X_INCREMENT);
        start_y = as3dp(start_y + Y_INCREMENT);
    }
    println!("Done!");
    let failures_size = failures.len() as i32;
    println!("Success: {:?}.  Failure {:?}", ITERATIONS - failures_size, failures_size);

    for failure in &failures {
        println!("Error ❌ {:?}", failure);
    }
}


fn print_locationset(location_set: LocationSet) {
    print_locationset_debug(location_set, false);
}

fn print_locationset_debug(location_set: LocationSet, debug: bool) {
    println!();
    println!("---------------------------------");
    println!("BNG:    {:?}", location_set.bng);
    println!("BNGv2:  {:?}", location_set.bng_v2);
    println!("ETRS89: {:?}", location_set.etrs89);

    if debug {
        println!("DEBUG: BNG X: {:.32}", location_set.bng.x());
        println!("DEBUG: BNG Y: {:.32}", location_set.bng.y());

        println!("DEBUG: BNGv2 X: {:.32}", location_set.bng_v2.x());
        println!("DEBUG: BNGv2 Y: {:.32}", location_set.bng_v2.y());

        println!("DEBUG: ETRS89 X: {:.32}", location_set.etrs89.x());
        println!("DEBUG: ETRS89 Y: {:.32}", location_set.etrs89.y());
    }

    println!("---------------------------------");
}

fn get_location_set(bng: Point<f64>, bng_v2: Point<f64>, etrs89: Point<f64>) -> LocationSet {
    return LocationSet {
        bng,
        bng_v2,
        etrs89,
    };
}

fn get_location_set_for_etrs(lat: f64, lon: f64, bng: &mut Proj, bng_v2: &mut Proj) -> Result<LocationSet, BngError> {
    // BNG
    let etrs89_to_bng = to_bng_ting(lat, lon, bng);
    return get_location_set_for_bng(etrs89_to_bng.x(), etrs89_to_bng.y(), bng, bng_v2);
}

fn get_location_set_for_bngv2(x: f64, y: f64, bng: &mut Proj, bng_v2: &mut Proj) -> Result<LocationSet, BngError> {
    // ETRS89
    let v2_to_etrs89 = to_etrs89(x, y, bng_v2);

    // BNG
    let v2_to_bng = to_bng_ting(v2_to_etrs89.y(), v2_to_etrs89.x(), bng);

    return get_location_set_for_bng(v2_to_bng.x(), v2_to_bng.y(), bng, bng_v2);
}

fn get_location_set_for_bng(x: f64, y: f64, bng: &mut Proj, bng_v2: &mut Proj) -> Result<LocationSet, BngError> {

    // convert to ETRS89
    let base_etrs89 = to_etrs89(x, y, bng);
    //println!("base_etrs89: {:?}", base_etrs89);

    // and now convert to BNGv2
    let base_bngv2 = to_bng_ting(base_etrs89.y(), base_etrs89.x(), bng_v2);
    //println!("base_bngv2: {:?}", base_bngv2);

    // and now back to ETRS89
    let v2_to_etrs89 = to_etrs89(base_bngv2.x(), base_bngv2.y(), bng_v2);
    //println!("v2_etrs89: {:?}", v2_etrs89);

    // and now back to BNG as BNG'
    let v2_to_bng = to_bng_ting(v2_to_etrs89.y(), v2_to_etrs89.x(), bng);

    if x == v2_to_bng.x() && y == v2_to_bng.y() {

        let location_set = get_location_set(Point::new(x, y), base_bngv2, v2_to_etrs89);
        let result = check_location_set3(location_set, bng, bng_v2);
        return result;
    }
    return get_location_set_for_bng_with_iteration(x, y, bng, bng_v2);
}

fn get_location_set_for_bng_with_iteration(x: f64, y: f64, bng: &mut Proj, bng_v2: &mut Proj) -> Result<LocationSet, BngError>  {

    // convert to ETRS89
    let base_etrs89 = to_etrs89(x, y, bng);

    // and now convert to BNGv2
    let base_bngv2 = to_bng_ting(base_etrs89.y(), base_etrs89.x(), bng_v2);

    // and now back to ETRS89
    let v2_etrs89 = to_etrs89(base_bngv2.x(), base_bngv2.y(), bng_v2);

    // and now back to BNG as BNG'
    let v2_bng = to_bng_ting(v2_etrs89.y(), v2_etrs89.x(), bng);

    let undefined: Point<f64> = Point::new(std::f64::MAX, std::f64::MAX);

    let result;
    let result_etrs89;

    let mut delta_x = x - v2_bng.x();
    let mut delta_y = y - v2_bng.y();

    let mut adjusted_result = undefined;
    let mut adjusted_result_etrs89 = undefined;

    for try in 0..10 {
        let bngv2_adjusted = Point::new(as4dp(base_bngv2.x() + delta_x), as4dp(base_bngv2.y() + delta_y));
        let v2_etrs89_adjusted = to_etrs89(bngv2_adjusted.x(), bngv2_adjusted.y(), bng_v2);
        let bng_via_adjustment = to_bng_ting(v2_etrs89_adjusted.y(), v2_etrs89_adjusted.x(), bng);
        let bng_etrs89_adjusted = to_etrs89(bng_via_adjustment.x(), bng_via_adjustment.y(), bng);

        if x == bng_via_adjustment.x() && y == bng_via_adjustment.y() {
            adjusted_result = bngv2_adjusted;
            adjusted_result_etrs89 = v2_etrs89_adjusted;
            break;
        } else {
            println!("------------------");
            println!("attempt: {:?}", try);
            println!("TARGET BNG: {:?} {:?}", x, y);
            println!("THIS BNG:   {:?}", bng_via_adjustment);

            println!("Delta {:.4} {:.4}", delta_x, delta_y);
            println!("BNGv2  adjusted point {:?}", bngv2_adjusted);
            println!("BNGv2  adjusted point {:.4},{:.4}", bngv2_adjusted.x(), bngv2_adjusted.y());
            println!("ETRS89 adjusted point {:?}", v2_etrs89_adjusted);

            println!("BNG ESTR89  {:?}", bng_etrs89_adjusted);

            println!("Refining: x {:?} -> xa {:?} and y {:?} -> ya {:?}",
                     x,
                     bng_via_adjustment.x(),
                     y,
                     bng_via_adjustment.y());

            let new_delta_x;

            let bool_x_is_same = x == bng_via_adjustment.x();
            if !bool_x_is_same {
                let bool_x_delta_is_small = (x - bng_via_adjustment.x()).abs() <= 0.002;
                if bool_x_delta_is_small {
                    if bng_etrs89_adjusted.x() > v2_etrs89_adjusted.x()  {
                        new_delta_x = -0.0001;
                    } else {
                        new_delta_x = 0.0001;
                    }
                } else {
                    new_delta_x = as4dp(x - bng_via_adjustment.x());
                }
            } else {
                new_delta_x = 0.0;
            }

            let new_delta_y;
            let bool_y_is_same = y == bng_via_adjustment.y();
            if !bool_y_is_same {

                // why 0.002 rather than 0.001?  There are edge cases where a whole movement of
                // 0.001 up or down misses the value.
                // For example:
                // The values x: 142003.658 y: 24838.174 might flip flop between
                // a) x: 142003.658, y: 24838.173
                // b) x: 142003.658, y: 24838.175
                let bool_y_delta_is_small = (y - bng_via_adjustment.y()).abs() <= 0.002;
                if bool_y_delta_is_small {
                    if bng_etrs89_adjusted.y() > v2_etrs89_adjusted.y()  {
                        println!("🍏 +ve");
                        new_delta_y = -0.0001;
                    } else {
                        println!("🍎 -ve");
                        new_delta_y = 0.0001;
                    }
                } else {
                    println!("🦖 LARGE DELTA!");
                    new_delta_y = as4dp(y - bng_via_adjustment.y());
                    println!("🦖 OLD Y {:?} NEW Y {:?}", y, bng_via_adjustment.y());
                    println!("🦖 NEW DELTA CORRECTION {:.4}", new_delta_y);
                }
            } else {
                new_delta_y = 0.0;
            }
            println!("delta x - old {:?} -> new {:?}", delta_x, as4dp(delta_x + new_delta_x));
            println!("delta y - old {:?} -> new {:?}", delta_y, as4dp(delta_y + new_delta_y));
            delta_x = as4dp(delta_x + new_delta_x);
            delta_y = as4dp(delta_y + new_delta_y);
        }
    }

    if adjusted_result == undefined {
        println!("❌❌❌❌❌❌❌");

        let location_set = LocationSet {
            bng: Point::new(x, y),
            bng_v2: undefined,
            etrs89: undefined,
        };
        return Err(BngError::LocationSetError(location_set));
    }
    result = adjusted_result;
    result_etrs89 = adjusted_result_etrs89;

    let location_set = LocationSet {
        bng: Point::new(x, y),
        bng_v2: Point::new(result.x(), result.y()),
        etrs89: Point::new(result_etrs89.x(), result_etrs89.y()),
    };
    // let us to that final check
    check_location_set3(location_set, bng, bng_v2);
    Ok(location_set)
}

fn request_input_coordinates(input_type: &InputType) -> Option<Point<f64>> {
    let mut x = 0.0 as f64;
    let mut y = 0.0 as f64;

    loop {
        match input_type {
            InputType::BNG | InputType::BNGv2 => {
                println!("Enter eastings and northing values (X and Y coordinates)");
                println!("For example: 50000.0 60000.0");
            },
            _ => {
                println!("Enter latitude and longitude values");
                println!("For example: 52.1 -1.2");
            }
        }

        let mut current_input = String::new();
        match io::stdin().read_line(&mut current_input) {
            Ok(_n) => {}
            Err(error) => println!("error: {}", error),
        }

        current_input = current_input.trim().to_string();

        if current_input == "q" {
            return None::<Point<f64>>;
        }

        use regex::Regex;
        let re = Regex::new(r"(\d+.\d*)[, ]+(-?\d+.\d*)").unwrap();

        let cap = re.captures(&current_input).unwrap();

        match input_type {
            InputType::BNG | InputType::BNGv2 => {
                x = cap[1].to_string().parse().unwrap();
                y = cap[2].to_string().parse().unwrap();
                break
            }
            _ => {
                x = cap[2].to_string().parse().unwrap();
                y = cap[1].to_string().parse().unwrap();
                break;
            }
        }
    }

    match input_type {
        InputType::BNG => {
            x = as3dp(x);
            y = as3dp(y);
        }
        InputType::BNGv2 => {
            x = as4dp(x);
            y = as4dp(y);
        }
        _ => {} // IGNORE ETRS89
    }

    return Some(Point::new(x, y));
}

fn request_input_coordinate_type() -> InputType {
    loop {
        println!("Select the coordinate:");
        println!("1 - BNG [X Y]");
        println!("2 - BNGv2 [X Y]");
        println!("3 - ETRS89 [lat lon]");
        println!();
        println!("t - execute tests");
        println!();
        println!("Select 1, 2 or 3");

        let mut current_input = String::new();
        match io::stdin().read_line(&mut current_input) {
            Ok(_n) => {}
            Err(error) => println!("error: {}", error),
        }

        current_input = current_input.trim().to_string();

        if current_input == "1" {
            // TODO normally we'd just return here...what is the deal with rust 'cos it cannot work
            // out that such behaviour is valid.
            return InputType::BNG;
        } else if current_input == "2" {
            return InputType::BNGv2;
        } else if current_input == "3" {
            return InputType::ETRS89;
        } else if current_input == "t" {
            return InputType::TEST;
        }
    }
}

enum InputType {
    UNSPECIFIED,
    TEST,
    BNG,
    BNGv2,
    ETRS89
}

#[derive(Debug)]
pub enum BngError {
    LocationSetError(LocationSet)
}

impl fmt::Display for BngError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            BngError::LocationSetError(ref location_set) => write!(f, "({})", location_set),
        }

    }
}

fn check_location_set3(location_set: LocationSet, bng: &mut Proj, bng_v2: &mut Proj) -> Result<LocationSet, BngError> {
    // convert BNG to ETRS89
    let _base_etrs89 = to_etrs89(location_set.bng.x(), location_set.bng.y(), bng);

    // Question: does ETRS89 value convert to the BNG value?
    let to_bng_from_etrs89 = to_bng_ting(location_set.etrs89.y(), location_set.etrs89.x(), bng);
    let is_etrs89_good = location_set.bng.x() == to_bng_from_etrs89.x() &&
        location_set.bng.y() == to_bng_from_etrs89.y();

    // Question: does BNGv2 value convert to BNG value?
    let v2_etrs89 = to_etrs89(location_set.bng_v2.x(), location_set.bng_v2.y(), bng_v2);
    let v2_bng = to_bng_ting(v2_etrs89.y(), v2_etrs89.x(), bng); // and now back to BNG as BNG'
    let is_bng_v2_good = location_set.bng.x() == v2_bng.x() && location_set.bng.y() == v2_bng.y();

    if is_etrs89_good && is_bng_v2_good {
        //println!("👍👍👍👍👍👍👍 🌍🌍🌍");
        Ok(location_set)
    } else {
        println!("ETRS89: {:?} BNGv2: {:?}", is_etrs89_good, is_bng_v2_good);
        println!("❌❌❌❌❌❌❌ 🌍🌍🌍");
        return Err(BngError::LocationSetError(location_set));
    }
}

fn as3dp(n: f64) -> f64 {
    return (n * 1000.0).round() / 1000.0;
}

fn as4dp(n: f64) -> f64 {
    return (n * 10000.0).round() / 10000.0;
}

fn to_etrs89(x: f64, y: f64, proj: &mut Proj) -> Point<f64> {
    let result = proj.project(Point::new(x, y), true).unwrap();
    return Point::new((result.x() as f64).to_degrees(), (result.y() as f64).to_degrees());
}

fn to_bng_ting(lat: f64, lon: f64, proj: &mut Proj) -> Point<f64> {
    let p_52_0_ll = Point::new(lon.to_radians(), lat.to_radians());
    let result = proj.project(p_52_0_ll, false).unwrap();
    return Point::new(as3dp(result.x()),as3dp(result.y()));
}