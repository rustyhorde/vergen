#!/usr/bin/env fish
echo "**** Running docs for test_util"; and \
cargo doc -p test_util -F repo; and \
echo "**** Running docs for vergen-lib"; and \
cargo doc -p vergen-lib -F build,cargo,git,rustc,si; and \
echo "**** Running docs for vergen"; and \
cargo doc -p vergen -F build,cargo,emit_and_set,rustc,si; and \
echo "**** Running docs for vergen-git2"; and \
cargo doc -p vergen-git2 -F build,cargo,emit_and_set,rustc,si; and \
echo "**** Running docs for vergen-gitcl"; and \
cargo doc -p vergen-gitcl -F build,cargo,emit_and_set,rustc,si; and \
echo "**** Running docs for vergen-gix"; and \
cargo doc -p vergen-gix -F build,cargo,emit_and_set,rustc,si; and \
echo "**** Running docs for vergen-pretty"; and \
cargo doc -p vergen-pretty -F color,header,trace