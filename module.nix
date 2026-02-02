{ config, lib, pkgs, ... }:

let
  cfg = config.programs.njalla;
in
{
  options.programs.njalla = {
    enable = lib.mkEnableOption "njalla-cli domain management tool";

    package = lib.mkOption {
      type = lib.types.package;
      description = "The njalla-cli package to use";
    };

    secretsFile = lib.mkOption {
      type = lib.types.nullOr lib.types.path;
      default = null;
      example = "/var/lib/secrets/njalla/env";
      description = ''
        Path to environment file containing NJALLA_API_TOKEN.
        File should contain: NJALLA_API_TOKEN=your-token
      '';
    };
  };

  config = lib.mkIf cfg.enable {
    environment.systemPackages = [
      (if cfg.secretsFile != null then
        pkgs.writeShellScriptBin "njalla" ''
          set -a
          [ -f ${cfg.secretsFile} ] && source ${cfg.secretsFile}
          set +a
          exec ${cfg.package}/bin/njalla "$@"
        ''
      else
        cfg.package)
    ];
  };
}
