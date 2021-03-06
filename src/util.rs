//
// A rust binding for the GSL library by Guillaume Gomez (guillaume1.gomez@gmail.com)
//

pub fn subinterval_too_small(a1: f64, a2: f64, b2: f64) -> bool {
    let e = ::DBL_EPSILON;
    let u = ::DBL_MIN;

    let tmp = unsafe { (1f64 + 100f64 * e) * (a2.abs() + 1000f64 * u) };

    unsafe { a1.abs() <= tmp && b2.abs() <= tmp }
}