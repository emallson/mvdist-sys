extern crate libc;
use libc::{c_int, c_double};

extern "C" {
    fn __mvwrap_MOD_mvndist(n: *const c_int,
                            covrnc: *const c_double,
                            nu: *const c_int,
                            m: *const c_int,
                            lower: *const c_double,
                            constr: *const c_double,
                            upper: *const c_double,
                            infin: *const c_int,
                            delta: *const c_double,
                            maxpts: *const c_int,
                            abseps: *const c_double,
                            releps: *const c_double,
                            error: *mut c_double,
                            value: *mut c_double,
                            nevals: *mut c_int,
                            inform: *mut c_int);
}

fn arr<T: Sized>(s: &[T]) -> *const T {
    s.as_ptr()
}

fn flatten<T: Sized + Clone>(m: &[Vec<T>]) -> Vec<T> {
    let mut res = vec![];
    for &ref v in m {
        res.append(&mut v.clone());
    }
    res
}


pub fn mvdist(n: i32,
              covrnc: &[Vec<f64>],
              nu: i32,
              m: i32,
              lower: &[f64],
              constr: &[Vec<f64>],
              upper: &[f64],
              infin: &[i32],
              delta: &[f64],
              maxpts: i32,
              abseps: f64,
              releps: f64)
              -> (f64, f64, i32, i32) {
    let mut error = 0f64;
    let mut value = 0f64;
    let mut nevals = 0i32;
    let mut inform = 0i32;
    unsafe {
        __mvwrap_MOD_mvndist(&n,
                             flatten(covrnc).as_ptr(),
                             &nu,
                             &m,
                             arr(lower),
                             flatten(constr).as_ptr(),
                             arr(upper),
                             arr(infin),
                             arr(delta),
                             &maxpts,
                             &abseps,
                             &releps,
                             &mut error,
                             &mut value,
                             &mut nevals,
                             &mut inform);
    }
    (error, value, nevals, inform)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn mvtst_a() {
        let n = 4;
        let m = 5;
        let nu = 8i32;
        let mx = 100000i32;
        let abseps = 1e-5f64;
        let mut cov = Vec::with_capacity(n);
        let mut cns = Vec::with_capacity(n);
        let delta = vec![0f64; m];
        let infin = vec![2i32; m];
        let lower = vec![0f64; m];
        let upper = vec![1f64; m];
        for i in 0..n {
            {
                let mut col = vec![0.0; n];
                col[i] = 1.0;
                cov.push(col);
            }
            {

                let mut col = vec![0f64; m];
                col[i] = 1f64;
                col[n] = 1f64;
                cns.push(col);
            }
        }

        let (error, value, nevals, inform) = mvdist(n as i32,
                                                    &cov,
                                                    nu,
                                                    m as i32,
                                                    &lower,
                                                    &cns,
                                                    &upper,
                                                    &infin,
                                                    &delta,
                                                    mx,
                                                    abseps,
                                                    0f64);

        assert!(inform == 0);
        assert!(nevals > 0);
        assert!((value - 0.001).abs() < 0.0001);
        assert!(error <= abseps);
    }
}
