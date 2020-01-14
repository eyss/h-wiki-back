# H-Wiki

the holochain backend for H-Wiki, see [H-Wiki](#) for user interface.

Resources:
[markdown](https://www.markdownguide.org/basic-syntax/)
## Running
Before starting up the UI development, start up a Holochain Conductor with the H-Wiki DNA. Here's how:
Enter a nix shell:
```
nix-shell --run holo-wiki
```
This starts up the Conductor with a running instance of the DNA in it.

Leave this terminal open and running, as long as you're doing development.

##Testing
```
holo-wiki-test
```
## Building

To rebuild the DNA that holochain uses to run use the `hc` command:

```
nix-shell --run `hc package`
```

Stop the running conductor (ctrl + c) and rerun the above again if you make changes to the DNA.

## Updating

To update the holonix version (and therefore the holochain binaries) edit the holonix property of `config.nix`.
