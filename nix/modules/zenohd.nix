{
  config,
  lib,
  pkgs,
  ...
}:
let
  cfg = config.services.zenohd;
in
{
  options.services.zenohd = {
    enable = lib.mkEnableOption "zenohd";
    # TODO: add other options necessary for zenohd
  };

  config = lib.mkIf cfg.enable {
    systemd.services.zenohd = {
      description = "zenoh router";
      wantedBy = [ "multi-user.target" ];

      serviceConfig = {
        ExecStart = "${pkgs.zenoh.zenohd}/bin/zenohd";
        Restart = "on-failure";
        RestartSec = "1";
      };
    };
  };
}
