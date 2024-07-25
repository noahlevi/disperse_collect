# Test

## Task Specifications
```
Develop an application on Rust that allow to do this:

Disperse:

Being able to send ETH to multiples wallets at the same time using an smart contract BUT ALSO any ERC20 tokens. I can also select % instead of amount.

Collect:

Being able to collect ETH from multiples at the same time and send just to one wallet using an smart contrct but ALSO ERC20 tokens using % of the wallet.

Application needs to have an API where you can send your body and it will process it and return tx hash. Develop the smart contract using solidity under Foundry framework. Optimize it for gas as much. 
``` 

**Read-Only Files**
- .gitignore
- .gitmodules
- api/Cargo.toml
- api/Cargo.lock
- README.md

**Environment**  
First of all you have to make some required installations. To make it relevant to your OS please visit official websites: <br>

1) `Rust` programming language - [official download](https://www.rust-lang.org/tools/install). Make sure `cargo` also has been installed. 

2) Install `Foundary`:
```bash
$ juscurl -L https://foundry.paradigm.xyz | bash && foundryup
```

3) Also you need to install `solc` [official download](https://docs.soliditylang.org/en/latest/installing-solidity.html)

4) !Install! `truffle`:
```bash
$ npm install -g truffle
```

**Usage Example**
First of all complile your contract 
```bash
$ truffle init && truffle compile
```

Then to execute `main.rs` script first of all you need to make build. Go to `api` dir:

```bash
$ cargo build
```
After your build has been done, just execute build file with next command:
```bash
$ .target/debug/api
```
