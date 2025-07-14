const { ApiPromise, WsProvider } = require('@polkadot/api');

async function main() {
  const wsProvider = new WsProvider('wss://rpc.polkadot.io');
  const api = await ApiPromise.create({ provider: wsProvider });

  const hash = await api.rpc.chain.getBlockHash();
  const block = await api.rpc.chain.getBlock(hash);
  const encoded = block.toU8a();
  const blockNumber = block.block.header.number.toNumber().toLocaleString('en-US').replace(/,/g, '_');;
  const formattedSize = encoded.length.toLocaleString('en-US').replace(/,/g, '_');
  console.log(`Polkadot block ${blockNumber} SCALE-encoded size (toU8a): ${formattedSize} bytes`);

  await api.disconnect();
}

main().catch(console.error);