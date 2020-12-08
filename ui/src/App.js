import "./App.css";
import React from "react";
import { SigningCosmWasmClient } from "secretjs";
import { v4 as uuidv4 } from "uuid";

const CODE_ID = 1;
const CHIAN_ID = "enigma-pub-testnet-3";
const REST = "http://localhost:1337";
const RPC = "localhost:26657";

class App extends React.Component {
  constructor(props) {
    super(props);

    this.state = {
      keplrReady: false,
      polls: [],
      newPollText: "",
    };

    this.createNewPoll = this.createNewPoll.bind(this);
    this.vote = this.vote.bind(this);
  }

  async componentWillMount() {
    await this.setupKeplr();

    const contracts = await this.secretjs.getContracts(CODE_ID);
    for (const contract of contracts) {
      const poll = await this.secretjs.queryContractSmart(contract.address, {
        get_poll: {},
      });
      contract.poll = poll;
    }
    this.setState({ polls: contracts });

    const ws = new WebSocket(`ws://${RPC}/websocket`);

    ws.onopen = () => {
      ws.send(
        JSON.stringify({
          jsonrpc: "2.0",
          method: "subscribe",
          params: {
            query: `message.module='compute' AND message.code_id='${CODE_ID}' AND message.action='instantiate'`,
          },
          id: "banana", // jsonrpc id
        })
      );
    };

    ws.onmessage = async ({ data }) => {
      try {
        const { result } = JSON.parse(data);
        const address = result.events["message.contract_address"][0];
        const poll = await this.secretjs.queryContractSmart(address, {
          get_poll: {},
        });
        this.setState({ polls: this.state.polls.concat({ address, poll }) });
      } catch (e) {
        console.error(e.message);
      }
    };
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
      rpc: `http://${RPC}`,
      rest: REST,
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
      REST,
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
    try {
      const response = await this.secretjs.instantiate(
        CODE_ID,
        { poll: newPollText },
        uuidv4()
      );
      alert(JSON.stringify(response));
      this.setState({ newPollText: "" });
    } catch (error) {
      alert(error.message);
    }
  }

  async vote(pollAddress, yes) {
    try {
      const response = await this.secretjs.execute(pollAddress, {
        Vote: { yes },
      });
      alert(JSON.stringify(response));
    } catch (error) {
      alert(error.message);
    }
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
      <center>
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
        <table>
          <thead>
            <tr>
              <th>Poll</th>
              <th>Vote</th>
            </tr>
          </thead>
          <tbody>
            {this.state.polls.map((poll, idx) => (
              <tr key={idx}>
                <td>{poll.poll}</td>
                <td>
                  <button onClick={() => this.vote(poll.address, true)}>
                    Yes
                  </button>
                  <button onClick={() => this.vote(poll.address, false)}>
                    No
                  </button>
                </td>
              </tr>
            ))}
          </tbody>
        </table>
      </center>
    );
  }
}

export default App;
