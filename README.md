# H-Wiki

DNA holochain code for H-Wiki, see [H-Wiki-Front](https://github.com/eyss/h-wiki-front) for user interface.

H-Wiki is a hApp (holochain application) that allows groups and communities to create their own wiki-like repositories of information.

Each wiki is created with an initial administrator, that can grant `administrator` or `editor` roles to any other user that joins the hApp.

Design: https://hackmd.io/HQ0wjyjjTpK4yJ9FcAx0Iw

## Getting started

Before starting up the [UI development server](https://github.com/eyss/h-wiki-front), start up a Holochain Conductor with this H-Wiki DNA. Here's how:

1. Make sure you've installed the Nix package manager on your computer. Follow the instructions for your operating system at the [Holochain developer portal](https://developer.holochain.org/docs/install/).

2. Clone or download this project onto your computer.

3. In a terminal window, enter the project's directory and enter the command:

    ```
    $ nix-shell
    ```

    This will set up the Holochain development environment with all the tools necessary to build the H-Wiki DNA and run an instance of it.

4. Once the development environment is set up, your terminal prompt should turn green. You're now ready to start a local sim2h server, which manages connections between nodes.

    ```
    $ sim2h_server
    ```

    If the sim2h server starts, you should see a blank line with no messages or terminal prompt. Leave this running for as long as you're running the DNA.

5. Open a new terminal, enter the project's directory, and again enter the command:

    ```
    $ nix-shell
    ```

    It should go a lot faster the second time.

6. Compile your DNA:

    ```
    $ hc package
    ```

    This will take a while. Get up, stretch, drink a glass of water.

7. Run an instance of your DNA, using the supplied conductor configuration and agent key:

    ```
    $ holochain -c conductor-config.toml
    ```

    The agent key is pre-generated because this hApp uses the [Progenitor pattern](https://forum.holochain.org/t/progenitor/1063), which gives admin access to a particular agent key.

    Note: if you get an error message like this:

    ```
    Error while trying to boot from config: ConfigError("Could not load DNA file \"dist/h-wiki-back.dna.json\"")
    ```

    That means your project directory name is slightly different from expected. The easiest way to fix this is to rename the project directory to `h-wiki-back` and try to recompile.

    If you get an error message like this:

    ```
    Error while trying to boot from config: ErrorGeneric("Error while trying to create instance \"__H_Wiki\": Provided DNA hash does not match actual DNA hash! QmT5PKN2xq5QjgsrbpmLopkpEZnwoKJnuuYrH8bpgBZEFX != QmeXZXx8RHV7qV9cXpaYoXYj13kHEvxvNoT1Aa2T3kNs64")
    ```

    It means that your DNA has compiled to a slightly different hash than expected. (This can happen if an upstream library has been updated.) You can fix this by running the command:

    ```
    $ hc hash
    ```

    then editing the `conductor-config.toml`:

    1. Find the section labeled `[[dnas]]`.
    2. Replace the value of `hash` with the value reported by `hc hash`.

## Next steps

* **Use your own agent key.** The Progenitor pattern marks a certain agent as the DHT super-admin by specifying their key in the `app.json` file. We've pre-generated a key and included it in this repo to make it easier to run the app. If you want to specify your own, open up `app.json` and replace the value of the `"progenitor"` property with their public key. Make sure you recompile the DNA and change its hash in `conductor-config.toml`, because a new progenitor means the DNA's validation rules have changed!

* **Collaborate with friends.** This takes a little extra configuration and works best on a LAN.

    1. On the first participant's machine, follow the instructions above to get the sim2h server and first conductor running. Take note of the first participant's IP address.

    2. On the second participant's machine, create a new agent key. You need to do this because, if the DHT sees one agent with two source chains, it'll mark the agent as a bad actor.

    ```
    hc keygen -n -p keystore.key
    ```

    It will tell you the newly generated public key; copy it.

    3. On the second participant's machine, edit the `conductor-config.toml` file:

        1. Look for the `[[agents]]` section and change the value of `public_address` to your newly generated public key.

        2. Look for the `[network]` section and change the hostname in the value of `sim2h_url` to the first participant's machine's IP address.

    4. Start up the conductor on the second participant's machine _without_ first starting up the sim2h server. Because both participants are using the sim2h server on the first machine, they'll be able to talk to each other.

* **Persist the wiki.** Currently the conductor config is set up to use an in-memory store. Once all agents stop their running instances, the wiki will disappear. To play around with a permanent DHT, each agent should change their `conductor-config.toml` file to persist their source chain and DHT shard to device storage:

    1. Create a persistence directory called `persistence_store` in the project directory (normally you would put it in a more permanent spot, but this will be fine for testing).

    2. Find the section labeled `[instances.storage]`.

    3. Change the `type` to `'file'`.

    4. Add a line to specify the path to persist data to:

        ```
        path = 'persistence_store'
        ```

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

If you're using the sample `conductor-config.toml` file, as in the above instructions, edit it and replace the old DNA hash with the new one that `hc package` reports.

Stop the running conductor (ctrl + c) and rerun the above again if you make changes to the DNA.

## Updating to new holochain version

To update the holonix version (and therefore the holochain binaries) edit the holonix property of `config.nix`.
