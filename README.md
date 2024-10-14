# rhea

A simple project generator for Rust in a nix environment.

To use it add it to your flake inputs:

```nix
# flake.nix
{
    inputs = {
        rhea.url = "github:sciencefidelity/rhea";
    }
}
```

Now you can add it to your home packages:

```nix
# home.nix
{ inputs, pkgs, ... };

{
    home.packages = [
        inputs.rhea.packages.${pkgs.system}.default
    ];
}
```

Now the `rhea` command is available for your user and is a basic replacement for `cargo new`:

```bash
# make a binary project called foo-bar
rhea foo-bar
# this also works
rhea foo-bar --bin
# make a library project instead
rhea foo-bar --lib
# now have fun developing with Nix...
cd foo-bar
direnv allow
cargo run
nix build
```

## Motivation

Something like this probably exists somewhere on the internet. It was pretty easy to write a quick Rust app to do it. In a Nix environment we don't have packages for building software installed globally. Instead each directory contains a `shell.nix` or a `flake.nix` that defines the build system for the package. The `cargo` command is not available globally, we can do `nix run nixpkgs#cargo init foo-bar`, to get the cargo command temporarily to init a new package, but that doesn't give us a flake, and other basics that we need for a package built with nix flake.

This is currently heavily opinionated. It adds strict lints, and options to make smaller packages to `Cargo.toml`. It assumes you are using `direnv` with `nix-direnv` to run to nix shell automatically in the directory. And it has my GitHub config hard coded in `Cargo.toml`. In time I'll extend it with CLI options. For now it works great for my use case.
