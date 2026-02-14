# Crypto Wallet CLI

A command-line software to generate crypto wallets for Bitcoin and Solana.

## About

This is a command-line tool written in Rust for generating cryptocurrency wallets. It provides a simple and secure way to create new wallets for Bitcoin and Solana directly from your terminal.

## Installation

You can install the `crypto-wallet` CLI directly from the source:

```bash
cargo install --path .
```

This will install the `wallet` binary in your cargo bin directory.

## Usage

The `crypto-wallet` CLI has a main binary `wallet` which can be used with subcommands `bitcoin` and `solana`.

### Bitcoin Wallet

To generate a Bitcoin wallet, use the `bitcoin` subcommand.

**Generate a wallet from a random seed:**

```bash
wallet bitcoin --random
```

**Generate a wallet from a text seed:**

```bash
wallet bitcoin "my secret seed text"
```

**Generate a wallet from a SHA256 hash:**

```bash
wallet bitcoin --hash <your-sha256-hash>
```

**Generate a wallet from a file:**

```bash
wallet bitcoin --file /path/to/your/seed/file
```

**Options:**

*   `--compressed`: Generate a compressed public key.
*   `--print <value>`: Print only a specific value. Available values are `all`, `private_key`, `public_key`, `address`.
*   `--raw`: Print the raw value without any extra information.

### Solana Wallet

To generate a Solana wallet, use the `solana` subcommand.

**Generate a wallet from a random seed:**

```bash
wallet solana --random
```

**Generate a wallet from a text seed:**

```bash
wallet solana "my secret seed text"
```

**Generate a wallet from a SHA256 hash:**

```bash
wallet solana --hash <your-sha256-hash>
```

**Generate a wallet from a file:**

```bash
wallet solana --file /path/to/your/seed/file
```

**Options:**

*   `--print <value>`: Print only a specific value. Available values are `all`, `private_key`, `public_key`.
*   `--raw`: Print the raw value without any extra information.

### Shell Completion

The CLI can generate shell completions for `bash`, `zsh`, `fish`, etc.

```bash
wallet completion <your-shell>
```

## Contributing

Contributions are welcome! Please feel free to submit a pull request or open an issue.

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.
