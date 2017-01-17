extern crate gcc;

fn main() {
    gcc::compile_library("libmvdist.a",
                         &["src/fortran/mvwrap.f90", "src/fortran/mvdist.f90"]);
    println!("cargo:rustc-link-lib=dylib=gfortran");
    println!("cargo:rustc-link-lib=dylib=gfortranbegin");
}
