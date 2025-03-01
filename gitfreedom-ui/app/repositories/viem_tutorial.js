import { createPublicClient, http } from 'viem';

//Use the command "node viem_tutorial.js" to run and test the code

// Use a local network instead of mainnet
const client = createPublicClient({
  transport: http('http://127.0.0.1:8547') // Local node URL
});

async function getBlockNumber() {
  try {
    const blockNumber = await client.getBlockNumber();
    console.log(`Latest Block Number: ${blockNumber}`);
  } catch (error) {
    console.error('Error fetching block number:', error);
  }
}

getBlockNumber();
