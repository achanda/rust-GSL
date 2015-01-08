//
// A rust binding for the GSL library by Guillaume Gomez (guillaume1.gomez@gmail.com)
//

#![macro_use]

macro_rules! rgsl_error(
    ($msg:expr, $err_value:expr) => (
        {
            let file = file!();

            unsafe {
                $msg.with_c_str(|c_str|{
                    file.with_c_str(|c_file|{
                        ffi::gsl_error(c_str, c_file, line!() as i32, $err_value as i32)
                    });
                });
            }
        }
    );
);