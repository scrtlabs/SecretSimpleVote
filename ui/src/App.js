import "./App.css";
import React from "react";
import { SigningCosmWasmClient } from "secretjs";

const CODE_ID = 1;
const CHIAN_ID = "enigma-pub-testnet-3";

class App extends React.Component {
  constructor(props) {
    super(props);

    this.state = {
      keplrReady: false,
      polls: [],
      newPollText: "",
    };

    this.createNewPoll = this.createNewPoll.bind(this);
  }

  async componentWillMount() {
    await this.setupKeplr();

    setInterval(async () => {
      const contracts = await this.secretjs.getContracts(CODE_ID);
      for (const contract of contracts) {
        const poll = await this.secretjs.queryContractSmart(contract.address, {
          get_poll: {},
        });
        contract.poll = poll;
      }
      this.setState({ polls: contracts });
    }, 1000);
  }

  async setupKeplr() {
    const sleep = (ms) => new Promise((accept) => setTimeout(accept, ms));

    while (
      !window.keplr &&
      !window.getOfflineSigner &&
      !window.getEnigmaUtils
    ) {
      await sleep(10);
    }

    await window.keplr.experimentalSuggestChain({
      chainId: CHIAN_ID,
      chainName: "Local Secret Chain",
      rpc: "http://localhost:26657",
      rest: "http://localhost:8010/proxy",
      bip44: {
        coinType: 529,
      },
      coinType: 529,
      stakeCurrency: {
        coinDenom: "SCRT",
        coinMinimalDenom: "uscrt",
        coinDecimals: 6,
      },
      bech32Config: {
        bech32PrefixAccAddr: "secret",
        bech32PrefixAccPub: "secretpub",
        bech32PrefixValAddr: "secretvaloper",
        bech32PrefixValPub: "secretvaloperpub",
        bech32PrefixConsAddr: "secretvalcons",
        bech32PrefixConsPub: "secretvalconspub",
      },
      currencies: [
        {
          coinDenom: "SCRT",
          coinMinimalDenom: "uscrt",
          coinDecimals: 6,
        },
      ],
      feeCurrencies: [
        {
          coinDenom: "SCRT",
          coinMinimalDenom: "uscrt",
          coinDecimals: 6,
        },
      ],
      gasPriceStep: {
        low: 0.1,
        average: 0.25,
        high: 0.4,
      },
      features: ["secretwasm"],
    });

    await window.keplr.enable(CHIAN_ID);

    this.keplrOfflineSigner = window.getOfflineSigner(CHIAN_ID);
    this.accounts = await this.keplrOfflineSigner.getAccounts();

    this.secretjs = new SigningCosmWasmClient(
      "http://localhost:8010/proxy",
      this.accounts[0].address,
      this.keplrOfflineSigner,
      window.getEnigmaUtils(CHIAN_ID),
      {
        init: {
          amount: [{ amount: "300000", denom: "uscrt" }],
          gas: "300000",
        },
        exec: {
          amount: [{ amount: "300000", denom: "uscrt" }],
          gas: "300000",
        },
      }
    );

    this.setState({ keplrReady: true });
  }

  async createNewPoll() {
    const newPollText = this.state.newPollText;
    this.setState({ newPollText: "" });

    await this.secretjs.instantiate(
      CODE_ID,
      { poll: newPollText },
      newPollText
    );
  }

  render() {
    if (!this.state.keplrReady) {
      return (
        <>
          <h1>Waiting for Keplr wallet integration...</h1>
        </>
      );
    }

    return (
      <>
        <h1>Create Poll</h1>
        <form>
          <input
            name="poll"
            value={this.state.newPollText}
            onChange={({ target }) => {
              this.setState({ newPollText: target.value });
            }}
          />
          <button type="button" onClick={this.createNewPoll}>
            Submit
          </button>
        </form>
        <h1>Polls</h1>
        {this.state.polls.map((p) => (
          <h4>{JSON.stringify(p)}</h4>
        ))}
      </>
    );
  }
}

export default App;
