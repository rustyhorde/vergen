// Dogfooding it, used in tests mostly.
extern crate vergen;

use vergen::{ConstantsFlags, Result, Vergen};

fn main() {
    gen_constants().expect("Unable to generate vergen constants!");
}

fn gen_constants() -> Result<()> {
    let vergen = Vergen::new(ConstantsFlags::all())?;

    for (k, v) in vergen.build_info() {
        println!("cargo:rustc-env={}={}", k.name(), v);
    }

    Ok(())
}
