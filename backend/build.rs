use vergen_git2::{BuildBuilder, Emitter, Git2Builder};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let git2 = Git2Builder::all(&mut Default::default()).describe(true, true, None).build()?;
    let build = BuildBuilder::all_build()?;
    Emitter::default()
        .add_instructions(&git2)?
        .add_instructions(&build)?
        .emit()?;
    Ok(())
}