# This file was generated by nvfetcher, please do not modify it manually.
{ fetchgit, fetchurl, fetchFromGitHub, dockerTools }:
{
  hypridle = {
    pname = "hypridle";
    version = "4395339a2dc410bcf49f3e24f9ed3024fdb25b0a";
    src = fetchFromGitHub {
      owner = "hyprwm";
      repo = "hypridle";
      rev = "4395339a2dc410bcf49f3e24f9ed3024fdb25b0a";
      fetchSubmodules = false;
      sha256 = "sha256-ZSn3wXQuRz36Ta/L+UCFKuUVG6QpwK2QmRkPjpQprU4=";
    };
    date = "2024-03-11";
  };
  hyprlock = {
    pname = "hyprlock";
    version = "23224d40e4ee7ed2e9d4be831724071a305c77d6";
    src = fetchFromGitHub {
      owner = "hyprwm";
      repo = "hyprlock";
      rev = "23224d40e4ee7ed2e9d4be831724071a305c77d6";
      fetchSubmodules = false;
      sha256 = "sha256-YY8grq5bv3kr6/QLa27xenthQxgMnXVyxtAfhnQHAik=";
    };
    date = "2024-03-17";
  };
  path-of-building = {
    pname = "path-of-building";
    version = "v2.39.3";
    src = fetchFromGitHub {
      owner = "PathOfBuildingCommunity";
      repo = "PathOfBuilding";
      rev = "v2.39.3";
      fetchSubmodules = false;
      sha256 = "sha256-W4MmncDfeiuN7VeIeoPHEufTb9ncA3aA8F0JNhI9Z/o=";
    };
  };
  swww = {
    pname = "swww";
    version = "24cc0c34c3262bee688a21070c7e41e637c03d71";
    src = fetchFromGitHub {
      owner = "LGFae";
      repo = "swww";
      rev = "24cc0c34c3262bee688a21070c7e41e637c03d71";
      fetchSubmodules = false;
      sha256 = "sha256-QfIHfB1/5PTWHSWnwORmDsfAQzuvkbggoQm2YixY6ZU=";
    };
  };
  wallust = {
    pname = "wallust";
    version = "104d99fcb4ada743d45de76caa48cd899b021601";
    src = fetchgit {
      url = "https://codeberg.org/explosion-mental/wallust.git";
      rev = "104d99fcb4ada743d45de76caa48cd899b021601";
      fetchSubmodules = false;
      deepClone = false;
      leaveDotGit = false;
      sha256 = "sha256-gGyxRdv2I/3TQWrTbUjlJGsaRv4SaNE+4Zo9LMWmxk8=";
    };
    date = "2024-03-08";
  };
}
