cargo doc -p test_util -F repo &&
cargo doc -p vergen-lib -F build,cargo,git,rustc,si &&
cargo doc -p vergen -F build,cargo,emit_and_set,rustc,si &&
cargo doc -p vergen-git2 -F build,cargo,emit_and_set,rustc,si &&
cargo doc -p vergen-gitcl -F build,cargo,emit_and_set,rustc,si &&
cargo doc -p vergen-gix -F build,cargo,emit_and_set,rustc,si &&
cargo doc -p vergen-pretty -F color,header,trace