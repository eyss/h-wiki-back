/// NB: The try-o-rama config patterns are still not quite stabilized.
/// See the try-o-rama README [https://github.com/holochain/try-o-rama]
/// for a potentially more accurate example

const path = require("path");

const {
  Orchestrator,
  Config,
  tapeExecutor,
  singleConductor,
  localOnly,
  combine,
  callSync
} = require("@holochain/tryorama");

process.on("unhandledRejection", error => {
  // Will print "unhandledRejection err is not defined"
  console.error("got unhandledRejection:", error);
});

const dnaPath = path.join(__dirname, "../dist/holo_wiki.dna.json");

const globalConfig = {
  logger: {
    type: "info",
    rules: {
      rules: [
        {
          exclude: true,
          pattern: ".*parity.*"
        },
        {
          exclude: true,
          pattern: ".*mio.*"
        },
        {
          exclude: true,
          pattern: ".*tokio.*"
        },
        {
          exclude: true,
          pattern: ".*hyper.*"
        },
        {
          exclude: true,
          pattern: ".*rusoto_core.*"
        },
        {
          exclude: true,
          pattern: ".*want.*"
        },
        {
          exclude: true,
          pattern: ".*rpc.*"
        }
      ]
    },
    state_dump: false
  },
  network: {
    type: "sim2h",
    sim2h_url: "ws://localhost:9000" // 'ws://public.sim2h.net:9000'
  } // Config.network('memory')
};

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

    // must use singleConductor middleware if using in-memory network
    tapeExecutor(require("tape")),
    localOnly,
    singleConductor
    // callSync
  )
});

const dna = Config.dna(dnaPath, "holo_wiki");
const fullConfig = Config.gen({ app: dna }, globalConfig);

orchestrator.registerScenario("create profile test", async (s, t) => {
  // the 'true' is for 'start', which means boot the Conductors
  const { alice } = await s.players({ alice: fullConfig }, true);

  await alice.call("app", "wiki", "create_page_with_elements", {
    title: "venezuela",
    contents: [
      {
        type: "p",
        content: "text",
        rendered_content: "hol"
      }
    ]
  });
  await s.consistency();

  const addr = await alice.call("app", "wiki", "get_page", {
    title: "venezuela"
  });
  await s.consistency();
  await alice.call("app", "wiki", "delete_element", {
    element_address: addr.Ok.sections[0].address
  });
  await s.consistency();
  await alice.call("app", "wiki", "get_page", {
    title: "venezuela"
  });
  await s.consistency();
  await alice.call("app", "wiki", "add_page_element", {
    element: {
      type: "ps",
      content: "texts",
      rendered_content: "hols"
    },
    title: "venezuela"
  });
  await s.consistency();
  const addrw = await alice.call("app", "wiki", "get_page", {
    title: "venezuela"
  });
  await s.consistency();
  await alice.call("app", "wiki", "delete_element", {
    element_address: addrw.Ok.sections[0].address
  });
  await s.consistency();

  await alice.call("app", "wiki", "get_page", {
    title: "venezuela"
  });
  const addr3 = await alice.call("app", "wiki", "get_home_page", {});
});
orchestrator.run();
