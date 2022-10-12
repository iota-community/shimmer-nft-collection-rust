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

2. Get your address:

```bash
cargo run --bin get-address
```

Now you need to add tokens to the address. You can use the [Shimmer Faucet](https://faucet.testnet.shimmer.network/) in the testnet.

3. Create Collection NFT

```bash
cargo run --bin create-collection-nft
```

4. Mint NFTs

```bash
cargo run --bin create-nfts
```

5. Send NFTs to addresses in the CSV file.

```bash
cargo run --bin send-nfts-to-csv < drop.csv
```
