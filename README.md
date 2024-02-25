# Uniswap V2 AMM implemented in Anchor

- `programs/amm/src/draft.rs`: outline of program with comments -- drafted before implementation

## Supported Instructions

- `programs/amm/src/`
    - `instructions/`
        - `init_pool.rs`: initialize a new pool
        - `liqduidity.rs`: add and remove liquidity
        - `swap.rs`: perform a token swap

## Implemented Tests

- `tests/amm.ts`:
    - initialize a new pool
    - add liquidity (x3)
    - remove liquidity
    - swap
    - remove liquidity

# develop env

```
$ rm -rf $HOME/.cache/solana
$ sh -c "$(curl -sSfL https://release.solana.com/v1.18.3/install)"
$ solana --version
solana-cli 1.18.3 (src:6f13e1c2; feat:3352961542, client:SolanaLabs)
$ solana config set --url localhost
$ solana address
$ solana-keygen new
```

```
$ cargo install --git https://github.com/project-serum/anchor anchor-cli --locked 
$ anchor --version
anchor-cli 0.29.0
```

```
$ anchor build --arch sbf
```

```
$ anchor test --arch sbf
```

# reference

```
https://lorisleiva.com/create-a-solana-dapp-from-scratch/getting-started-with-solana-and-anchor 
```





```shell
$ cargo version
cargo 1.76.0 (c84b36747 2024-01-18)
$ rustc --version
rustc 1.76.0 (07dca489a 2024-02-04)
```

```shell
$ solana --version
solana-cli 1.18.3 (src:6f13e1c2; feat:3352961542, client:SolanaLabs)
```

```shell
$ solana-test-validator --version
solana-test-validator 1.18.3 (src:6f13e1c2; feat:3352961542, client:SolanaLabs)
```

```shell
$ anchor --version   
anchor-cli 0.29.0
```

```shell
$ node --version
v20.11.1
```

```shell
$ npm --version
10.4.0
```

```shell
$ yarn --version
1.22.21
```

* 编译

```shell
$ anchor build
```

* 运行单元测试

```shell
$ anchor test
```

* 启动 solana 本地测试节点

```shell
$ solana-test-validator
```

* 部署

```shell
$ anchor deploy
```


