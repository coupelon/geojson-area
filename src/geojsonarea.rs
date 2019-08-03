/*
This code is a simple adaptation to Rust of the code provided by mapbox here :
https://github.com/mapbox/geojson-area/blob/master/index.js

Note : it doesn't exactly match the 'contenance' values hardcoded in the
geojson.io dataset from cadastre.gouv.fr 
*/

use geojson::{Geometry, Value};
use geojson::PolygonType;
use geojson::Position;

pub fn geometry(geom: Geometry) -> f64 {
    match geom.value {
        Value::Polygon(polygon) => {
            return polygon_area(polygon);
        }
        Value::MultiPolygon(multipoly) => {
            let mut area = 0.0;
            for poly in multipoly {
                area += polygon_area(poly);
            }
            return area;
        }
        Value::GeometryCollection(collection) => {
            let mut area = 0.0;
            for cl in collection {
                area += geometry(cl);
            }
            return area;
        }
        // Point, LineString, and their Multi– counterparts
        _ => {
            return 0.0;
        }
    }
}

fn polygon_area(coords :PolygonType) -> f64 {
    let mut area = 0.0;
    if coords.len() > 0 {
        area += ring_area(coords[0].clone()).abs();
        for i in 1..coords.len() {
            area -= ring_area(coords[i].clone()).abs();
        }
    }
    return area;
}

/**
 * Calculate the approximate area of the polygon were it projected onto
 *     the earth.  Note that this area will be positive if ring is oriented
 *     clockwise, otherwise it will be negative.
 *
 * Reference:
 * Robert. G. Chamberlain and William H. Duquette, "Some Algorithms for
 *     Polygons on a Sphere", JPL Publication 07-03, Jet Propulsion
 *     Laboratory, Pasadena, CA, June 2007 http://trs-new.jpl.nasa.gov/dspace/handle/2014/40409
 *
 * Returns:
 * {float} The approximate signed geodesic area of the polygon in square
 *     meters.
 */

fn ring_area(coords :Vec<Position>) -> f64 {
    let mut area = 0.0;
    //From https://github.com/mapbox/wgs84/blob/master/index.js
    let wgs84_radius = 6378137.0;
    let coords_length = coords.len();

    if coords_length > 2 {
        for i in 0..coords_length {
            let mut _lower_index = 0;
            let mut _middle_index = 0;
            let mut _upper_index = 0;
            if i == coords_length - 2 {// i = N-2
                _lower_index = coords_length - 2;
                _middle_index = coords_length -1;
                _upper_index = 0;
            } else if i == coords_length - 1 {// i = N-1
                _lower_index = coords_length - 1;
                _middle_index = 0;
                _upper_index = 1;
            } else { // i = 0 to N-3
                _lower_index = i;
                _middle_index = i+1;
                _upper_index = i+2;
            }
            let p1 = coords[_lower_index].clone();
            let p2 = coords[_middle_index].clone();
            let p3 = coords[_upper_index].clone();
            area += ( rad(p3[0]) - rad(p1[0]) ) * rad(p2[1]).sin();
        }

        area = area * wgs84_radius * wgs84_radius / 2.0;
    }

    return area;
}

fn rad(tmp :f64) -> f64 {
    tmp * std::f64::consts::PI / 180.0
}