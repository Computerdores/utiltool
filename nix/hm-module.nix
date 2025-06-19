{ config, lib, pkgs, utiltoolPkg, ... }:

let
    cfg = config.programs.utiltool;
in {
    options.programs.utiltool = {
        enable = lib.mkEnableOption "Enable utiltool.";
        pick_file_script = lib.mkOption {
            type = lib.types.str;
            default = ''
                t=$(mktemp -t 'utiltool-pick-file.XXXXXX')
                yazi --chooser-file=$t
                cat $t
                rm $t
            '';
            description = "A bash script to pick one or more files. Should output one file path per line to stdout.";
        };
    };

    config = lib.mkIf cfg.enable {
        home.packages = [
            utiltoolPkg
        ];

        xdg.configFile."utiltool/config.yaml".text = lib.generators.toYAML {} {
            pick_file_script = cfg.pick_file_script;
        };
    };
}