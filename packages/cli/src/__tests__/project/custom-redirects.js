const { ethereumPlugin } = require("@web3api/ethereum-plugin-js");
const { ipfsPlugin } = require("@web3api/ipfs-plugin-js");

module.exports = [
  {
    from: "w3://ens/ethereum.web3api.eth",
    to: ethereumPlugin({
      networks: {
        rinkeby: {
          provider: "https://rinkeby.infura.io/v3/b00b2c2cc09c487685e9fb061256d6a6",
        },
      },
      defaultNetwork: "rinkeby"
    }),
  },
  {
    from: "w3://ens/ipfs.web3api.eth",
    to: ipfsPlugin({
      provider: "https://ipfs.io",
    }),
  }
];