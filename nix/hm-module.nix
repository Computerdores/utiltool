args@{ config, pkgs, utiltoolPkg, ... }:

let
    cfg = config.programs.utiltool;
    lib = import ./lib.nix args;
in {
    options.programs.utiltool = {
        enable = lib.mkEnableOption "Enable utiltool.";
        pick_file_script = lib.mkOption {
            type = lib.types.nullOr lib.types.str;
            default = null;
            example = ''
                t=$(mktemp -t 'utiltool-pick-file.XXXXXX')
                yazi --chooser-file=$t
                cat $t
                rm $t
            '';
            description = "A bash script to pick one or more files. Should output one file path per line to stdout.";
        };

        wallpaper_script = lib.mkOption {
            type = lib.types.nullOr lib.types.str;
            default = null;
            example = ''
                hyprctl hyprpaper preload $1 >/dev/null
                hyprctl hyprpaper wallpaper ,$1 >/dev/null
            '';
            description = "A bash script to set the wallpaper. Will receive the path of the wallpaper as the first argument.";
        };

        system = {
            shutdown_script = lib.mkOption {
                type = lib.types.nullOr lib.types.str;
                default = null;
                example = "shutdown now";
                description = "A bash script to shutdown the system.";
            };

            reboot_script = lib.mkOption {
                type = lib.types.nullOr lib.types.str;
                default = null;
                example = "reboot";
                description = "A bash script to reboot the system.";
            };

            hibernate_script = lib.mkOption {
                type = lib.types.nullOr lib.types.str;
                default = null;
                example = "systemctl hibernate";
                description = "A bash script to hibernate the system. Before running this script, the lock script will be run as well.";
            };

            suspend_script = lib.mkOption {
                type = lib.types.nullOr lib.types.str;
                default = null;
                example = "systemctl suspend";
                description = "A bash script to suspend the system. Before running this script, the lock script will be run as well.";
            };

            logout_script = lib.mkOption {
                type = lib.types.nullOr lib.types.str;
                default = null;
                example = "uwsm stop";
                description = "A bash script to logout of the current wayland session.";
            };

            lock_script = lib.mkOption {
                type = lib.types.nullOr lib.types.str;
                default = null;
                example = "hyprctl --immediate >/dev/null";
                description = "A bash script to lock the current wayland session.";
            };
        };
    };

    config = lib.mkIf cfg.enable {
        home.packages = [
            utiltoolPkg
        ];

        xdg.configFile."utiltool/config.yaml".text = lib.generators.toYAML {} (lib.filterAttrsRec (k: v: v != null && k != "enable") cfg);
    };
}