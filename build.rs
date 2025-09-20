use vergen_gitcl::{CargoBuilder, Emitter, GitclBuilder, RustcBuilder};

fn main() {
    let cargo = CargoBuilder::all_cargo().expect("Failed to emit VERGEN_CARGO_* instructions");
    let gitcl = GitclBuilder::all_git().expect("Failed to emit VERGEN_GIT_* instructions");
    let rustc = RustcBuilder::all_rustc().expect("Failed to emit VERGEN_RUSTC_* instructions");

    Emitter::default()
        .add_instructions(&cargo)
        .expect("Failed to add VERGEN_CARGO_* instructions")
        .add_instructions(&gitcl)
        .expect("Failed to add VERGEN_GIT_* instructions")
        .add_instructions(&rustc)
        .expect("Failed to add VERGEN_RUSTC_* instructions")
        .emit()
        .expect("Failed to run vergen");
}
