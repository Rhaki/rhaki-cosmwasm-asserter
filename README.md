## Asserter

This contract has different uses:

# Debug and Simulation

Simulate tx is a really powerfull tools since allow to analize logs without change the state of the chain.

However one of the biggest limitations is the possibility to perform queries after the simulation.

To solve this, this contract accept a list of query to be performed to a specified contracts, and add to result of each query to the log.

# Assert

Another powerfull way to use this contract is to query a value to a contract and assert this value.

For example, if after a msg that change the state of the chain, you want to revert the tx based on the value of a querable state of a contract, you can do this (for example if after a tx the balance of a cw20 token in a specific wallet is lesser then a value, the tx is reverted).

Currently this assert method con be done only vs other contract query response, (you can't assert a balance of native token, or the current block or timestamp) but this will be implemented in the future.

