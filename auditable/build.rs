use std::env;
use std::fs::File;
use std::io::Write;
use std::path::Path;

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();
    let dest_dir = Path::new(&out_dir);
    let mut f = File::create(dest_dir.join("Cargo.lock.annotated")).unwrap();
    // FIXME: this breaks if `CARGO_TARGET_DIR` env variable is overridden
    let cargo_lock_location = dest_dir.join("../../../../../Cargo.lock");
    let stuff_to_write = std::fs::read_to_string(cargo_lock_location).unwrap();
    write!(&mut f, "CARGO_AUDIT_INFO_START;v0;\n{}\nCARGO_AUDIT_INFO_END\0", stuff_to_write).unwrap();
}
