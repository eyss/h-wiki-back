{ pkgs }:
let
  script = pkgs.writeShellScriptBin "holo-wiki"
  ''
  set -euxo pipefail
  hc test
  '';
in
{
 buildInputs = [ script ];
}
