# H-Wiki

DNA holochain code for H-Wiki, see [H-Wiki](https://github.com/eyss/h-wiki-front) for user interface.

H-Wiki is a hApp (holochain application) that allows groups and communities to create their own wiki-like repositories of information.

Each wiki is created with an initial administrator, that can grant `administrator` or `editor` roles to any other user that joins the hApp.

Design: https://hackmd.io/HQ0wjyjjTpK4yJ9FcAx0Iw

## Running

Before starting up the UI development, start up a Holochain Conductor with the H-Wiki DNA. Here's how:
Enter a nix shell inside of the `h-wiki-back` folder:

```
nix-shell
hc run
```

This starts up the Conductor with a running instance of the DNA - which is stored in `dist/h-wiki-back` in it.

Leave this terminal open and running, as long as you're doing development.

## Testing

Run this command to run the tests (right now only minimal tests):

```
hc test
```

## Building

To rebuild the DNA that holochain uses to run use the `hc` command:

```
hc package
```

Stop the running conductor (ctrl + c) and rerun the above again if you make changes to the DNA.

## Updating to new holochain version

To update the holonix version (and therefore the holochain binaries) edit the holonix property of `config.nix`.
