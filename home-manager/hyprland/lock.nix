{
  config,
  isLaptop,
  lib,
  pkgs,
  ...
}:
let
  lock_cmd = lib.getExe pkgs.custom.shell.lock;
in
lib.mkIf (config.custom.hyprland.enable && config.custom.hyprland.lock) {
  programs.hyprlock.enable = true;

  custom.shell.packages = {
    lock = {
      runtimeInputs = with pkgs; [
        killall
        hyprlock
      ];
      text = "killall hyprlock || hyprlock";
    };
  };

  wayland.windowManager.hyprland.settings = {
    bind = [ "$mod_SHIFT, x, exec, ${lock_cmd}" ];

    # handle laptop lid
    bindl = lib.mkIf isLaptop [ ",switch:Lid Switch, exec, ${lock_cmd}" ];
  };

  # lock on idle
  services.hypridle = {
    settings = {
      general = {
        inherit lock_cmd;
      };

      listener = [
        {
          timeout = 5 * 60;
          on-timeout = lock_cmd;
        }
      ];
    };
  };

  custom.wallust.templates."hyprlock.conf" = {
    text =
      let
        rgba = colorname: alpha: "rgba({{ ${colorname} | rgb }},${toString alpha})";
      in
      lib.hm.generators.toHyprconf {
        attrs = {
          general = {
            disable_loading_bar = false;
            grace = 0;
            hide_cursor = false;
          };

          background = map (mon: {
            monitor = "${mon.name}";
            # add trailing comment with monitor name for hypr-wallpaper to replace later
            path = "{{wallpaper}} # ${mon.name}";
            color = "${rgba "background" 1}";
          }) config.custom.monitors;

          input-field = {
            monitor = "";
            size = "300, 50";
            outline_thickness = 2;
            dots_size = 0.33;
            dots_spacing = 0.15;
            dots_center = true;
            outer_color = "${rgba "background" 0.8}";
            inner_color = "${rgba "foreground" 0.9}";
            font_color = "${rgba "background" 0.8}";
            fade_on_empty = false;
            placeholder_text = "";
            hide_input = false;

            position = "0, -20";
            halign = "center";
            valign = "center";
          };

          label = [
            {
              monitor = "";
              text = ''cmd[update:1000] echo "<b><big>$(date +"%H:%M")</big></b>"'';
              color = "${rgba "foreground" 1}";
              font_size = 150;
              font_family = "${config.custom.fonts.regular}";

              # shadow makes it more readable on light backgrounds
              shadow_passes = 1;
              shadow_size = 4;

              position = "0, 200";
              halign = "center";
              valign = "center";
            }
            {
              monitor = "";
              text = ''cmd[update:1000] echo "<b><big>$(date +"%A, %B %-d")</big></b>"'';
              color = "${rgba "foreground" 1}";
              font_size = 40;
              font_family = "${config.custom.fonts.regular}";

              # shadow makes it more readable on light backgrounds
              shadow_passes = 1;
              shadow_size = 2;

              position = "0, 60";
              halign = "center";
              valign = "center";
            }
          ];
        };
      };
    target = "${config.xdg.configHome}/hypr/hyprlock.conf";
  };
}
