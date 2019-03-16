let
#https://github.com/mozilla/nixpkgs-mozilla/issues/160
#moz_overlay = import (builtins.fetchTarball https://github.com/mozilla/nixpkgs-mozilla/archive/master.tar.gz);
#pkgs = import <nixpkgs> { overlays = [ moz_overlay ]; };
pkgs = import <nixpkgs> { };
in
  [
    pkgs.rustc
    pkgs.rustfmt
    pkgs.cargo
  ]
