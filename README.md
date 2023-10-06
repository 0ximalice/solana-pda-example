# Solana Program Derived Addresses (PDAs) Example

This repository provides a simple example of using Solana Program Derived Addresses (PDAs) in Solana blockchain development. PDAs are addresses that are derived from a program and a set of seeds. They are commonly used in Solana programs to create flexible and secure on-chain logic.

## Prerequisites

Before you begin, ensure you have met the following requirements:

- [Solana Command Line Tools](https://docs.solana.com/cli/installation)
- [Basic understanding of Rust programming language and Solana blockchain](https://www.rust-lang.org/learn/get-started)
- [Anchor Framework](https://project-serum.github.io/anchor/getting-started/installation.html) - A Solana development framework for creating, testing, and deploying smart contracts.

## Understanding the Example

This example demonstrates how to create a PDA using the Anchor framework and interact with it using Typescript. The code consists of two main parts:

**programs/pdas/src/lib.rs**: The Solana program logic created with the Anchor framework, which derive a PDA and performs actions with it. The key functions to pay attention to are `find_program_address` in the initialize method and `invoke_signed` in redeem method.

**tests/{initialize, redeem}.ts**, the client application demonstrates how to derive a PDA using Typescript, and perform actions with it. The main function demonstrates the usage of the program's functions.

## Getting Started

Follow these steps to get the project up and running:

1. Clone the repository:

```bash
git clone https://github.com/imalic3/solana-pda-example.git
cd solana-pda-example
```

2. Install dependencies:

```bash
# Install Solana CLI tools (if you haven't already)
sh -c "$(curl -fsSL https://release.solana.com/v1.16.3/install)"

# Install Anchor CLI (if you haven't already)
cargo install --git https://github.com/project-serum/anchor anchor-cli --locked

# Install TypeScript dependencies
yarn
```

4. Start a local Solana cluster:

```bash
solana-test-validator
solana config set --url localhost
```

5. In a new terminal window, run the following command to prepare account:

```bash
solana-keygen new -o ~/.config/solana/id.json
solana address
solana airdrop 100 {wallet}
```

5. Run the following command to deploy the program:

```bash
anchor build
anchor deploy
anchor run initialize
```

6. You would see the following output:

```bash
Using wallet: {wallet}
Program ID: A83dYxXd3yZ3bJoKdu7rgb63tvP7WDVgi1dxDhCWcwK
Your transaction signature: 5shn7JYNGhEe6kvxUTT3MMpF1sUMgWUJe7WZCprbdLfGt3RC5SZuRsxvy5n48vMx2ku2tyBXm5GwivD2CJ856bvN
    ✔ Is initialized (191ms)
```

7. Transfer 2 SOL to the PDA address:

```bash
solana transfer 4aaFxG7C7FpAuEKmV3DaBG2kbjg5PCGGAerHwmUTQEsN 2 --allow-unfunded-recipient
```

8. Redeem the 1 SOL from the PDA address:

```bash
anchor run redeem
```

9. You would see the following output:

```bash
Using treasury address (PDA): 4aaFxG7C7FpAuEKmV3DaBG2kbjg5PCGGAerHwmUTQEsN
Bump: 255
Your transaction signature: 2MsZPv2UUcmb5GoR6X5Em2nTs8DeBfPMfXxgrwXPaqcLd6u1X4v2z9r9tPaBxRoc8pPm1GNpuaHvjorz4MBoGDiP
    ✔ Is redeemed (154ms)
```
