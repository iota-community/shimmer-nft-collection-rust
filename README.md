# Create NFT Collection

> Mints a NFT Collection and sends NFT to addresses in a CSV list

## Run

0. Prepare `.env` file:
```bash
cp .env.example .env
```
Change the values in the new created `.env` file.

1. Create a new account:

```bash
cargo run --bin create-account
```

1. Create Collection NFT

```bash
cargo run --bin create-collection-nft
```
