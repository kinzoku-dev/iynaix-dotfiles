{ pkgs, user, config, ... }:
{
  config = {
    home-manager.users.${user} = {
      programs = {
        neovim = {
          enable = true;
          defaultEditor = true;
          viAlias = true;
          vimAlias = true;
          withNodeJs = true;
          withPython3 = true;
          extraPackages = with pkgs; [ fzf ];
        };
      };

      home = {
        file.".config/nvim" = {
          source = ./nvim;
          recursive = true;
        };
      };
    };

    iynaix.persist.home.directories = [
      ".vim"
      ".local/share/nvim" # data directory
      ".local/state/nvim" # persistent session info
    ];
  };
}
