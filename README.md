# Uniswap V2 AMM implemented in Anchor

- `programs/ammv2/src/draft.rs`: outline of program with comments -- drafted before implementation

## Supported Instructions

- `programs/ammv2/src/`
    - `instructions/`
        - `init_pool.rs`: initialize a new pool
        - `liqduidity.rs`: add and remove liquidity
        - `swap.rs`: perform a token swap

## Implemented Tests

- `tests/ammv2.ts`:
    - initialize a new pool
    - add liquidity (x3)
    - remove liquidity
    - swap
    - remove liquidity 




# develop env
```
$ rm -rf $HOME/.cache/solana
$ sh -c "$(curl -sSfL https://release.solana.com/v1.14.6/install)"
$ solana --version
solana-cli 1.16.5 (src:26777419; feat:2891131721, client:SolanaLabs)
$ solana config set --url localhost
$ solana address
$ solana-keygen new
```

```
$ cargo install --git https://github.com/project-serum/anchor anchor-cli --locked 
$ anchor --version
anchor-cli 0.27.0
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

