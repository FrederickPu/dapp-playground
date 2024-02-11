# Zero Knowledge comment section

Simple Comment Section contract
Comment Section is a common multi-party computation example, where each person can comment exactly once (in the form of 8bit unsigned integer) and no one knows which user made which comment.
This implementation works in following steps:
1. Initialization on the blockchain.
2. Receival of multiple secret messages, using the real zk protocol.
3. Once enough salaries have been received, the contract owner can start the ZK computation.
4. The Zk computation concatenates all of the messages together.
5. Once the zk computation is complete, the contract will publicize the the concatonated message variable.
6. Once the summed variable is public, the contract will compute the concatonated message and store it in
   the state, such that the value can be read by all.

**NOTE**: This contract is missing several features that a production ready contract should
possess, including:
- An allowlist over salarymen.
- Check that each address only sends a single variable.
- Sort message by integer size before concatonating them in order to ensure that who commented what can't be inferred by looking at the posting order.
