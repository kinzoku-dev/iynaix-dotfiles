{
  config,
  host,
  isNixOS,
  lib,
  pkgs,
  user,
  ...
}: let
  wallpapers_dir = "${config.xdg.userDirs.pictures}/Wallpapers";
  wallpapers_proj = "/persist${config.home.homeDirectory}/projects/wallpaper-utils";
  # crop wallpaper before displaying with swww
  swww-crop = pkgs.writeShellApplication {
    name = "swww-crop";
    runtimeInputs = with pkgs; [swww imagemagick];
    text = ''
      convert "$1" -crop "$2" - | swww img --outputs "$3" "''${@:4}" -;
    '';
  };
  # backup wallpapers to secondary drive
  wallpapers-backup = pkgs.writeShellApplication {
    name = "wallpapers-backup";
    runtimeInputs = with pkgs; [rsync];
    text = ''
      rsync -aP --delete --no-links "${wallpapers_dir}" "/media/6TBRED"
      # update rclip database
      ${lib.optionalString config.custom.rclip.enable ''
        cd "${wallpapers_dir}"
        rclip -f "cat" >  /dev/null
        cd - > /dev/null
      ''}
    '';
  };
  # sync wallpapers with laptop
  wallpapers-remote = pkgs.writeShellApplication {
    name = "wallpapers-remote";
    runtimeInputs = with pkgs; [rsync wallpapers-backup];
    text = ''
      wallpapers-backup
      rsync -aP --delete --no-links -e "ssh -o StrictHostKeyChecking=no" "${wallpapers_dir}" "${user}@''${1:-${user}-framework}:${config.xdg.userDirs.pictures}"
    '';
  };
  # process wallpapers with upscaling and vertical crop
  wallpapers-process = pkgs.writeShellApplication {
    name = "wallpapers-process";
    runtimeInputs = [wallpapers-backup];
    text = ''
      cd ${wallpapers_proj}
      # activate direnv
      direnv allow && eval "$(direnv export bash)"
      python main.py "$@"
      cd - > /dev/null
      wallpapers-backup
    '';
  };
  # choose vertical crop for wallpapper
  wallpapers-choose = pkgs.writeShellApplication {
    name = "wallpapers-choose";
    text = ''
      cd ${wallpapers_proj}
      # activate direnv
      direnv allow && eval "$(direnv export bash)"
      python choose.py "$@"
      cd - > /dev/null
    '';
  };
  # search wallpapers with rclip
  wallpapers-search = pkgs.writeShellApplication {
    name = "wallpapers-search";
    runtimeInputs = with pkgs; [rclip imv];
    text = ''
      cd "${wallpapers_dir}"
      rclip -f "$@" | imv;
      cd - > /dev/null
    '';
  };
in
  lib.mkMerge [
    (lib.mkIf (host == "desktop") {
      home.packages = [
        wallpapers-backup
        wallpapers-choose
        wallpapers-remote
        wallpapers-process
      ];

      gtk.gtk3.bookmarks = [
        "file://${wallpapers_proj}/in Walls In"
      ];

      programs.imv.settings.binds = {
        m = "exec mv \"$imv_current_file\" ${wallpapers_proj}/in; next";
      };
    })

    # TODO: rofi rclip?
    (lib.mkIf config.custom.rclip.enable {
      home.packages = [
        wallpapers-search
        pkgs.rclip
      ];

      home.shellAliases = {
        wallrg = "wallpapers-search";
      };

      custom.persist = {
        home.directories = [
          ".cache/clip"
        ];
        cache = [
          ".local/share/rclip"
        ];
      };
    })
    (lib.mkIf isNixOS {
      home.packages = [
        pkgs.swww
        swww-crop
      ];
    })
    {
      home.shellAliases = {
        current-wallpaper = "command cat ${config.xdg.cacheHome}/current_wallpaper";
      };
    }
  ]
