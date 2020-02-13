{ pkgs }:
let
  script = pkgs.writeShellScriptBin "holo-wiki"
  ''
  set -euxo pipefail
  holochain -c ./conductor-config.toml 
  '';
in
{
 buildInputs = [ script ];
}
