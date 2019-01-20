#!/usr/bin/env bash
run() {
  nix run -f "$(dirname "$0")/default.nix" -c "${@}"
}

setup() {
  run rustup default stable
}

rustc() {
  run rustc "${@}"
}
"$1" "${@:2}"
