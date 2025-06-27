# SPL-TERMINAL
A command-line tool for managing spl tokens on the solana blockchain. You can **mint**, **transfer**, and **burn** tokens directly from your terminal - no GUI required (Default RPC is set to be devnet). This project is built with **Rust** and **Solana SDKs**, this project is open-source and contributions are welcomed!

---

## Features
- Create new SPL token mints
- Transfer tokens to other wallets
- Burn tokens
- CLI support with subcommands (using `clap`)
- Keypair-based wallet interaction

---

## Requirements
- [Rust](https://rust-lang-org) (1.70+)
- Solana CLI installed (`solana --version`)
- A funded solana keypair (Devnet or Mainnet)

---

## How to use!
You can get started in two ways:

### Option 1: Run directly with Cargo
CLone the repository and run a command:

```bash
git clone https://github.com/Kym0211/SPL-TERMINAL.git
cd SPL-TERMINAL
cargo run -- <SUBCOMMAND> [OPTIONS]
```

Example - 

```bash
cargo run -- create-mint --rpc https://api.devnet.solana.com --keypair ~/.config/solana/id.json
```

### Option 2: Install as a CLI tool
Install the tool globally:

```bash
cargo install --path .
```

Now you can use it like a regular terminal command:

```bash
spl create-mint --keypair ~/.config/solana/id.json
```

## Commands
Below are the available commands you can use with `spl-terminal`:

### Create a new mint 
Create a new SPL token mint.
```bash
    spl create-mint \
    --rpc https://api.devnet.solana.com \        # optional (defaults to devnet)
    --keypair ~/.config/solana/devnet.json       # required
```

### Mint tokens in an account 
Mint new tokens to a recipient's associated token account.

```bash
    spl mint-to \
        --rpc https://api.devnet.solana.com \       # optional (defaults to devnet)
        --keypair ~/.config/solana/devnet.json \
        --mint <MINT_ADDRESS> \
        --recipient <RECIPIENT_WALLET_ADDRESS> \
        --amount <NUMBER_OF_TOKENS>

```

### Transfer tokens to an account 
Transfer SPL tokens from one wallet to another.

```bash
    spl mint-to \
        --rpc https://api.devnet.solana.com \       # optional (defaults to devnet)
        --keypair ~/.config/solana/devnet.json \
        --mint <MINT_ADDRESS> \
        --destination <RECIPIENT_WALLET_ADDRESS> \
        --amount <NUMBER_OF_TOKENS>

```

### Burn tokens from an account 
Burn tokens from the owner’s associated token account.
```bash
    spl mint-to \
        --rpc https://api.devnet.solana.com \       # optional (defaults to devnet)
        --keypair ~/.config/solana/devnet.json \
        --mint <MINT_ADDRESS> \
        --amount <NUMBER_OF_TOKENS>

```