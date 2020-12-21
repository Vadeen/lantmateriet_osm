//! Math to convert from sweref 99 to WGS 84 coordinate systems.
//! See: https://www.lantmateriet.se/globalassets/kartor-och-geografisk-information/gps-och-geodetisk-matning/gauss_conformal_projection.pdf

use std::f64::consts::PI;
use vadeen_osm::geo::Coordinate;

const AXIS: f64 = 6_378_137.0; // GRS 80
const FLATTENING: f64 = 1.0 / 298.257_222_101; // GRS 80
const CENTRAL_MERIDIAN: f64 = 15.00;
const SCALE: f64 = 0.9996;
const FALSE_NORTHING: f64 = 0.0;
const FALSE_EASTING: f64 = 500_000.0;
const E2: f64 = FLATTENING * (2.0 - FLATTENING);
const N: f64 = FLATTENING / (2.0 - FLATTENING);
const A: f64 = AXIS / (1.0 + N) * (1.0 + N * N / 4.0 + N * N * N * N / 64.0);

const PHI1: f64 = N / 2.0 - 2.0 * N * N / 3.0 + 37.0 * N * N * N / 96.0 - N * N * N * N / 360.0;
const PHI2: f64 = N * N / 48.0 + N * N * N / 15.0 - 437.0 * N * N * N * N / 1440.0;
const PHI3: f64 = 17.0 * N * N * N / 480.0 - 37.0 * N * N * N * N / 840.0;
const PHI4: f64 = 4397.0 * N * N * N * N / 161_280.0;

const A_STAR: f64 = E2 + E2 * E2 + E2 * E2 * E2 + E2 * E2 * E2 * E2;
const B_STAR: f64 = -(7.0 * E2 * E2 + 17.0 * E2 * E2 * E2 + 30.0 * E2 * E2 * E2 * E2) / 6.0;
const C_STAR: f64 = (224.0 * E2 * E2 * E2 + 889.0 * E2 * E2 * E2 * E2) / 120.0;
const D_STAR: f64 = -(4279.0 * E2 * E2 * E2 * E2) / 1260.0;

const DEG_TO_RAD: f64 = PI / 180.0;
const LAMBDA_ZERO: f64 = CENTRAL_MERIDIAN * DEG_TO_RAD;

pub fn to_wgs(north: f64, east: f64) -> Coordinate {
    let xi = (north - FALSE_NORTHING) / (SCALE * A);
    let eta = (east - FALSE_EASTING) / (SCALE * A);
    let xi_prim = xi
        - PHI1 * (2.0 * xi).sin() * (2.0 * eta).cosh()
        - PHI2 * (4.0 * xi).sin() * (4.0 * eta).cosh()
        - PHI3 * (6.0 * xi).sin() * (6.0 * eta).cosh()
        - PHI4 * (8.0 * xi).sin() * (8.0 * eta).cosh();
    let eta_prim = eta
        - PHI1 * (2.0 * xi).cos() * (2.0 * eta).sinh()
        - PHI2 * (4.0 * xi).cos() * (4.0 * eta).sinh()
        - PHI3 * (6.0 * xi).cos() * (6.0 * eta).sinh()
        - PHI4 * (8.0 * xi).cos() * (8.0 * eta).sinh();
    let phi_star = (xi_prim.sin() / eta_prim.cosh()).asin();
    let delta_lambda = (eta_prim.sinh() / xi_prim.cos()).atan();
    let lon_radian = LAMBDA_ZERO + delta_lambda;
    let lat_radian = phi_star
        + phi_star.sin()
            * phi_star.cos()
            * (A_STAR
                + B_STAR * ((phi_star.sin()).powi(2))
                + C_STAR * (phi_star.sin().powi(4))
                + D_STAR * (phi_star.sin().powi(6)));

    let lon = lon_radian * 180.0 / PI;
    let lat = lat_radian * 180.0 / PI;
    Coordinate::new(lat, lon)
}
