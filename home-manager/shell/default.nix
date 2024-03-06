{ pkgs, ... }:
{
  imports = [
    ./bash.nix
    ./btop.nix
    ./cava.nix
    ./direnv.nix
    ./eza.nix
    ./fish.nix
    ./git.nix
    ./neovim
    ./nix.nix
    ./rice.nix
    ./shell.nix
    ./starship.nix
    ./tmux.nix
    ./yazi.nix
  ];

  home.packages = with pkgs; [
    # dysk # better disk info
    fd
    fx
    htop
    jq
    sd
    ugrep
  ];

  programs = {
    bat.enable = true;

    fzf = {
      enable = true;
      enableBashIntegration = true;
      enableFishIntegration = true;
    };

    zoxide = {
      enable = true;
      enableBashIntegration = true;
      enableFishIntegration = true;
      options = [ "--cmd cd" ];
    };
  };

  custom.persist = {
    home = {
      cache = [ ".local/share/zoxide" ];
    };
  };
}
