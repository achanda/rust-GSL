//
// A rust binding for the GSL library by Guillaume Gomez (guillaume1.gomez@gmail.com)
//

use ffi;
use c_vec::CVec;

pub struct FftComplexWaveTable<'a> {
    w: *mut ffi::gsl_fft_complex_wavetable,
    f: CVec<'a, u64>
}

impl<'a> FftComplexWaveTable<'a> {
    /// This function prepares a trigonometric lookup table for a complex FFT of length n. The function returns a pointer to the newly allocated
    /// gsl_fft_complex_wavetable if no errors were detected, and a null pointer in the case of error. The length n is factorized into a product
    /// of subtransforms, and the factors and their trigonometric coefficients are stored in the wavetable. The trigonometric coefficients are
    /// computed using direct calls to sin and cos, for accuracy. Recursion relations could be used to compute the lookup table faster, but if
    /// an application performs many FFTs of the same length then this computation is a one-off overhead which does not affect the final throughput.
    /// 
    /// The wavetable structure can be used repeatedly for any transform of the same length. The table is not modified by calls to any of the other
    /// FFT functions. The same wavetable can be used for both forward and backward (or inverse) transforms of a given length.
    pub fn new(n: u64) -> Option<FftComplexWaveTable<'a>> {
        let tmp = unsafe { ffi::gsl_fft_complex_wavetable_alloc(n) };

        if tmp.is_null() {
            None
        } else {
            unsafe {
                Some(FftComplexWaveTable {
                    w: tmp,
                    f: CVec::new((*tmp).factor.as_mut_ptr(), 64us)
                })
            }
        }
    }

    pub fn factor<'r>(&'r mut self) -> &'r mut [u64] {
        self.f.as_mut_slice()
    }
}

#[unsafe_destructor]
impl<'a> Drop for FftComplexWaveTable<'a> {
    fn drop(&mut self) {
        unsafe { ffi::gsl_fft_complex_wavetable_free(self.w) };
        self.w = ::std::ptr::null_mut();
    }
}

impl<'a> ffi::FFI<ffi::gsl_fft_complex_wavetable> for FftComplexWaveTable<'a> {
    fn wrap(w: *mut ffi::gsl_fft_complex_wavetable) -> FftComplexWaveTable<'a> {
        unsafe {
            FftComplexWaveTable {
                w: w,
                f: CVec::new((*w).factor.as_mut_ptr(), 64us)
            }
        }
    }

    fn unwrap(w: &FftComplexWaveTable) -> *mut ffi::gsl_fft_complex_wavetable {
        w.w
    }
}

pub struct FftComplexWorkspace {
    w: *mut ffi::gsl_fft_complex_workspace
}

impl FftComplexWorkspace {
    /// This function allocates a workspace for a complex transform of length n.
    pub fn new(n: u64) -> Option<FftComplexWorkspace> {
        let tmp = unsafe { ffi::gsl_fft_complex_workspace_alloc(n) };

        if tmp.is_null() {
            None
        } else {
            Some(FftComplexWorkspace {
                w: tmp
            })
        }
    }
}

impl Drop for FftComplexWorkspace {
    fn drop(&mut self) {
        unsafe { ffi::gsl_fft_complex_workspace_free(self.w) };
        self.w = ::std::ptr::null_mut();
    }
}

impl ffi::FFI<ffi::gsl_fft_complex_workspace> for FftComplexWorkspace {
    fn wrap(w: *mut ffi::gsl_fft_complex_workspace) -> FftComplexWorkspace {
        FftComplexWorkspace {
            w: w
        }
    }

    fn unwrap(w: &FftComplexWorkspace) -> *mut ffi::gsl_fft_complex_workspace {
        w.w
    }
}