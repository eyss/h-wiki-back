{ pkgs }:
let
  script = pkgs.writeShellScriptBin "holo-wiki-test"
  ''
  set -euxo pipefail
  hc test
  '';
in
{
 buildInputs = [ script ];
}
