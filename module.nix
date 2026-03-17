{ lib, pkgs, config, ... }:

let
  cfg = config.services.nixos-config-puller;
in
{
  options.services.nixos-config-puller = {
    enable = lib.mkEnableOption "nixos-config-puller";

    package = lib.mkOption {
      type = lib.types.package;
      default = pkgs.callPackage ./default.nix {};
      description = "The nixos-config-puller package to use.";
    };

    user = lib.mkOption {
      type = lib.types.str;
      default = "";
      description = "User account that should run nixos-config-puller at login. Empty disables autostart service.";
    };

    autoStart = lib.mkOption {
      type = lib.types.bool;
      default = true;
      description = "Whether to autostart nixos-config-puller for the configured user.";
    };
  };

  config = lib.mkIf cfg.enable {
    environment.systemPackages = [ cfg.package ];

    systemd.user.services.nixos-config-puller = lib.mkIf (cfg.autoStart && cfg.user != "") {
      description = "NixOS Config Puller";

      wantedBy = [ "default.target" ];
      after = [ "graphical-session.target" "network-online.target" ];
      wants = [ "network-online.target" ];

      environment = {
        XDG_SESSION_TYPE = "wayland";
        WAYLAND_DISPLAY = "wayland-0";
      };

      serviceConfig = {
        ExecStart = "${cfg.package}/bin/nixos-config-puller";
        Restart = "on-failure";
        RestartSec = 5;
      };
    };
  };
}
