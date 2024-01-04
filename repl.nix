# better repl with preloaded functions and libs already loaded
# https://bmcgee.ie/posts/2023/01/nix-and-its-slow-feedback-loop/#how-you-should-use-the-repl
{
  # host is passed down from the nrepl via a --arg argument, defaulting to the current host
  host ? "desktop",
  ...
}: let
  user = "iynaix";
  flake = builtins.getFlake (toString ./.);
  inherit (flake.inputs.nixpkgs) lib;
in rec {
  inherit (flake) inputs self;
  inherit (flake.inputs) nixpkgs;
  inherit flake lib host user;

  # default host
  c = flake.nixosConfigurations.${host}.config;
  co = c.iynaix-nixos;
  inherit (c) hm;
  hmo = hm.iynaix;
  inherit (flake.nixosConfigurations.${host}) pkgs;

  desktop = flake.nixosConfigurations.desktop.config;
  desktopo = desktop.iynaix-nixos;
  desktopHm = desktop.hm;
  desktopHmo = desktopHm.iynaix;

  framework = flake.nixosConfigurations.framework.config;
  frameworko = framework.iynaix-nixos;
  frameworkHm = framework.hm;
  frameworkHmo = frameworkHm.iynaix;

  laptop = flake.nixosConfigurations.framework.config;
  laptopo = framework.iynaix-nixos;
  laptopHm = framework.hm;
  laptopHmo = frameworkHm.iynaix;

  vm = flake.nixosConfigurations.vm.config;
  vmo = vm.iynaix-nixos;
  vmHm = vm.hm;
  vmHmo = vmHm.iynaix;

  # your code here
}
