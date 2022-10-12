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

2. Create Collection NFT

```bash
cargo run --bin create-collection-nft
```

3. Mint NFTs

```bash
cargo run --bin create-nfts
```


cargo run --bin send-nfts-to-csv < drop.csv