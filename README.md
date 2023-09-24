# Concordium-counter

Simple counter smart contract

## Build

```sh
cargo concordium build --out dist/module.wasm.v1 --schema-out dist/schema.bin
```

## Query status of transaction

```sh
tx_id=...
concordium-client transaction status $tx_id --grpc-port 20000 --grpc-ip node.testnet.concordium.com
```

## Deploy

```sh
cargo concordium build --out dist/module.wasm.v1 --schema-out dist/schema.bin
```

## Init

```sh
concordium-client contract init <YOUR-MODULE-HASH> --sender <YOUR-ADDRESS> --energy 30000 --contract counter --grpc-port 20000 --grpc-ip node.testnet.concordium.com
```

## Interact

### View state

```sh
concordium-client contract invoke <YOUR-CONTRACT-INSTANCE> --entrypoint view --schema dist/schema.bin --grpc-port 20000 --grpc-ip node.testnet.concordium.com
```

### Increment

```sh
echo -n "42" > dist/operator.json
concordium-client contract update <YOUR-CONTRACT-INSTANCE> --entrypoint increment --parameter-json <PATH-TO-JSON> --schema dist/schema.bin --sender <YOUR-ADDRESS> --energy 6000 --grpc-port 20000 --grpc-ip node.testnet.concordium.com
```

### Decrement

```sh
echo -n "-5" > dist/operator.json
concordium-client contract update <YOUR-CONTRACT-INSTANCE> --entrypoint decrement --parameter-json <PATH-TO-JSON> --schema dist/schema.bin --sender <YOUR-ADDRESS> --energy 6000 --grpc-port 20000 --grpc-ip node.testnet.concordium.com
```
