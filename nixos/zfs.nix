{
  config,
  lib,
  user,
  ...
}:
let
  cfg = config.custom-nixos.zfs;
  persistCfg = config.custom-nixos.persist;
in
lib.mkIf cfg.enable {
  boot = {
    # booting with zfs
    supportedFilesystems = [ "zfs" ];
    kernelPackages = config.boot.zfs.package.latestCompatibleLinuxPackages;
    zfs = {
      devNodes = lib.mkDefault "/dev/disk/by-id";
      enableUnstable = true;
      requestEncryptionCredentials = cfg.encryption;
    };
  };

  services.zfs = {
    autoScrub.enable = true;
    trim.enable = true;
  };

  # 16GB swap
  swapDevices = [ { device = "/dev/disk/by-label/SWAP"; } ];

  # standardized filesystem layout
  fileSystems =
    let
      homeMountPoint =
        if
          persistCfg.erase.home
        # using config.hm.homeDirectory results in infinite recursion
        then
          "/home/${user}"
        else
          "/home";
    in
    {
      # boot partition
      "/boot" = {
        device = "/dev/disk/by-label/NIXBOOT";
        fsType = "vfat";
      };

      # zfs datasets
      "/" = {
        device = "zroot/root";
        fsType = "zfs";
        neededForBoot = !persistCfg.tmpfs && cfg.erase.root;
      };

      "/nix" = {
        device = "zroot/nix";
        fsType = "zfs";
      };

      "/tmp" = {
        device = "zroot/tmp";
        fsType = "zfs";
      };

      "${homeMountPoint}" = {
        device = "zroot/home";
        fsType = "zfs";
        neededForBoot = !persistCfg.tmpfs && cfg.erase.home;
      };

      "/persist" = {
        device = "zroot/persist";
        fsType = "zfs";
        neededForBoot = true;
      };

      "/persist/cache" = {
        device = "zroot/cache";
        fsType = "zfs";
        neededForBoot = true;
      };
    };

  services.sanoid = lib.mkIf cfg.snapshots {
    enable = true;

    datasets = {
      "zroot/home" = lib.mkIf (!persistCfg.erase.home) {
        hourly = 50;
        daily = 15;
        weekly = 3;
        monthly = 1;
      };

      "zroot/persist" = {
        hourly = 50;
        daily = 15;
        weekly = 3;
        monthly = 1;
      };
    };
  };
}
