# Stellar Guestbook DApp

**Stellar Guestbook DApp** - Decentralized Message & Greeting System on Stellar

## Project Description

Stellar Guestbook DApp is a decentralized smart contract application built on the Stellar blockchain using the Soroban SDK. It provides an open, immutable, and transparent platform where users from around the world can leave public messages, greetings, or feedback directly on the blockchain. By eliminating centralized database dependencies, all guestbook entries are permanently secured on-chain.

The system allows users to broadcast public messages, retrieve historical records, and listen to real-time events upon every new submission, leveraging the high efficiency and low latency of the Stellar network.

## Project Vision

Our vision is to redefine digital guestbooks and public interaction platforms by:

* **Decentralizing Public Records**: Moving community guestbooks from central Web2 servers to a global, distributed ledger.
* **Ensuring True Immutability**: Providing a permanent, tamper-proof record of public interactions that cannot be censored or deleted by centralized authorities.
* **Enabling Real-Time Interaction**: Harnessing Soroban RPC event emissions for instant, live user interactions across decentralized frontends.
* **Fostering Trustless Communication**: Creating an environment where message authenticity is verified directly by cryptographic signatures, not middleman platforms.

## Key Features

### 1. **Message Submission (`write_message`)**

* Leave public messages on-chain with a simple transaction call.
* Automated timestamping via Stellar ledger metadata.
* Built-in validation to prevent empty submissions.
* Multi-wallet authorization using Stellar account signatures.

### 2. **Global Message Retrieval (`get_messages`)**

* Read all stored guestbook entries in a single query.
* Structured data presentation (sender address, message string, timestamp).
* Instant state synchronization with the Stellar network.

### 3. **Real-Time Event Integration**

* Emits `new_msg` events upon every valid message submission.
* Real-time frontend updates via Soroban RPC event listeners without page refreshes.

### 4. **Multi-Wallet & Error Handling**

* Seamlessly connects with popular Stellar wallets (Freighter, xBull, Albedo, LOBSTR).
* Robust error handling for user rejections, empty inputs, and insufficient gas balances.

## Contract Details

* Contract Address: `PASTE_YOUR_DEPLOYED_CONTRACT_ADDRESS_HERE`

## Future Scope

### Short-Term Enhancements

1. **Rich Text & Emoji Support**: Support formatted text and extended character sets for richer messages.
2. **Paging & Limits**: Implement pagination for fetching messages to optimize gas and RPC load as the entry count grows.
3. **Tipping Integration**: Allow visitors to attach XLM or custom SAC tokens alongside their guestbook entries as a tip to the host.

### Medium-Term Development

4. **NFT Badges / Proof-of-Visit**: Mint a unique commemorative NFT badge to every user who signs the guestbook.
5. **Profanity & Spam Filtering**: Integrate decentralized oracle verification to filter out unwanted spam or malicious content.
6. **Multi-Threaded Replies**: Allow users to comment or reply directly to specific existing guestbook entries.

### Long-Term Vision

7. **Cross-Chain Guestbook**: Expand contract deployment to other WASM and EVM-compatible chains with unified frontend state indexing.
8. **DAO & Protocol Governance**: Allow community members to vote on guestbook curation and featured entries using protocol tokens.

---

## Technical Requirements

* Soroban SDK
* Rust programming language
* Stellar blockchain network (Testnet / Mainnet)
* Stellar Wallets Kit (Frontend integration)

## Getting Started

Deploy the smart contract to Stellar's Soroban network and interact with it using the core functions:

* `write_message()` - Append a new greeting entry and publish an event.
* `get_messages()` - Retrieve the complete list of guestbook entries from persistent storage.

---

**Stellar Guestbook DApp** - Leaving Your Permanent Mark on the Blockchain