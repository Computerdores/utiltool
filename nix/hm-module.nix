{ config, lib, pkgs, utiltoolPkg, ... }:

let
    cfg = config.programs.utiltool;
in {
    options.programs.utiltool = {
        enable = lib.mkEnableOption "Enable utiltool.";
    };

    config = lib.mkIf cfg.enable {
        home.packages = [
            utiltoolPkg
        ];
    };
}