#!/usr/bin/env bash
run() {
  if [ $# -eq 0 ]
  then
    nix run -f "$(dirname "$0")/default.nix"
  else
    nix run -f "$(dirname "$0")/default.nix" -c "${@}"
  fi
}

rustc() {
  run rustc "${@}"
}
"$1" "${@:2}"
