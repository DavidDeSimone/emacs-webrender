{ lib, config, pkgs, ... }:
let
  cfg = config.language.rust;
  date = "2021-04-25";
in
with pkgs;
with lib;
{
  options.language.rust = {
    rustOverlay = mkOption {
      type = types.attrs;
      default = pkgs.rust-bin.nightly."${date}";
      description = "Which nightly rust version to use
      check valid data from https://github.com/oxalica/rust-overlay/tree/master/manifests/nightly";
    };

    rustPackages = mkOption {
      type = types.attrs;
      default = pkgs.rustPackages;
      description = "Which rust package set to use";
    };

    rustPackagesSet = mkOption {
      type = types.listOf types.str;
      default = [
      ];
      description = "Which rust tools to pull from the nixpkgs channel package set
      search valid packages here https://search.nixos.org/packages?channel=unstable&";
    };

    rustOverlaySet = mkOption {
      type = types.listOf types.str;
      default = [
        "default"
        "rust-analysis"
        "rust-analyzer-preview"
      ];
      description = "Which rust tools to pull from the rust overlay
      https://github.com/oxalica/rust-overlay/blob/master/manifests/profiles.nix";
    };
  };

  config = {
    env = [{
      # Used by tools like rust-analyzer
      name = "RUST_SRC_PATH";
      value =
        let rust-src = rust-bin.nightly."${date}".default.override {
          extensions = [ "rust-src" ];
        };
        in
        (toString rust-src) + "/lib/rustlib/src/rust/library";
    }];

    devshell.packages = map (tool: cfg.rustPackages.${tool}) cfg.rustPackagesSet
      ++ map (tool: cfg.rustOverlay.${tool}) cfg.rustOverlaySet ++ (with pkgs;[
      #custom nixpkgs packages
      rustracer
    ]);
  };
}
