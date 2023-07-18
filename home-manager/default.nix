{
  user,
  pkgs,
  ...
}: {
  imports = [
    ./programs
    ./shell
  ];

  # mounting and unmounting of disks
  services.udiskie = {
    enable = true;
    automount = true;
    notify = true;
  };

  programs.direnv = {
    enable = true;
    nix-direnv.enable = true;
  };

  home = {
    username = user;
    homeDirectory = "/home/${user}";
    # do not change this value
    stateVersion = "22.11";

    packages = with pkgs; [
      libreoffice
    ];
  };
}
