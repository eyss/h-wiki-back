const path = require("path");

const {
  Orchestrator,
  Config,
  tapeExecutor,
  singleConductor,
  combine,
  callSync
} = require("@holochain/try-o-rama");

process.on("unhandledRejection", error => {
  // Will print "unhandledRejection err is not defined"
  console.error("got unhandledRejection:", error);
});

const dnaPath = path.join(__dirname, "../dist/holo_wiki.dna.json");

const orchestrator = new Orchestrator({
  middleware: combine(
    // squash all instances from all conductors down into a single conductor,
    // for in-memory testing purposes.
    // Remove this middleware for other "real" network types which can actually
    // send messages across conductors
    // singleConductor,
    // callSync,
    // use the tape harness to run the tests, injects the tape API into each scenario
    // as the second argument
    tapeExecutor(require("tape"))
  ),

  globalConfig: {
    logger: false,
    network: {
      type: "sim2h",
      sim2h_url: "wss://sim2h.holochain.org:9000"
    } // must use singleConductor middleware if using in-memory network
  }

  // the following are optional:
});

const conductorConfig = {
  instances: {
    holo_wiki: Config.dna(dnaPath, "holo_wiki")
  }
};
orchestrator.registerScenario("create profile test", async (s, t) => {
  // the 'true' is for 'start', which means boot the Conductors
  const { alice } = await s.players({ alice: conductorConfig }, true);
  const addr = await alice.call("holo_wiki", "wiki", "create_page", {
    tag: "venezuela"
  });
  await s.consistency();
  const addr2 = await alice.call("holo_wiki", "wiki", "create_page_elements", {
    elements: [
      {
        parent_page_anchor: addr.Ok,
        element_type: "p",
        element_content: "hola"
      }
    ],
    page_adress: addr.Ok
  });
  await s.consistency();
  const addr3 = await alice.call("holo_wiki", "wiki", "get_page", {
    address: addr.Ok
  });
  const addr4 = await alice.call("holo_wiki", "wiki", "get_elements_page", {
    address: addr.Ok
  });
  const addr6 = await alice.call("holo_wiki", "wiki", "add_page_element", {
    element: {
      parent_page_anchor: addr.Ok,
      element_type: "p",
      element_content: "hola2"
    },
    page_adress: addr.Ok
  });
  const addr5 = await alice.call("holo_wiki", "wiki", "get_elements_page", {
    address: addr.Ok
  });
});
orchestrator.run();
