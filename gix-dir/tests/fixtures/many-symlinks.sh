#!/bin/bash
set -eu -o pipefail

# Note that symlink creation fails on Windows for some reason,
# so these tests shouldn't be run there.

git init breakout-symlink
(cd breakout-symlink
  mkdir hide
  ln -s ../.. hide/breakout
  touch file
)

ln -s breakout-symlink symlink-to-breakout-symlink

git init immediate-breakout-symlink
(cd immediate-breakout-symlink
  ln -s .. breakout
)

git init excluded-symlinks-to-dir
(cd excluded-symlinks-to-dir
  cat <<EOF >.gitignore
src1
src2/
file1
file2/
ignored
ignored-must-be-dir/
EOF
  git add .gitignore && git commit -m "init"

  mkdir src
  >src/file

  mkdir ignored-must-be-dir ignored
  touch ignored-must-be-dir/file ignored/file

  ln -s src src1
  ln -s src src2
  ln -s src/file file1
  ln -s src/file file2
)
