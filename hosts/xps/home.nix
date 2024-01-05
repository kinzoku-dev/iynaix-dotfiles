_: {
  iynaix = {
    displays = [
      {
        name = "eDP-1";
        hyprland = "1920x1080,0x0,1";
        workspaces = [1 2 3 4 5 6 7 8 9 10];
      }
    ];

    wezterm.enable = false;
    terminal.size = 10;

    persist = {
      home.directories = [
        {
          directory = "Downloads";
          method = "symlink";
        }
      ];
    };
  };
}
