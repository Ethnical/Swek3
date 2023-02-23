## Swek the EVM Security Toolbox.

Swek is a Security toolbox focused on EVM written in Rust.

Most of the features are designed to help auditors to get faster in the daily works.

Feel free to contact me to add some features or found any bugs.

If you are intersted in Rust, MeV, Solidity Sec join us here => https://discord.gg/uxqbK7wS

![ezgif-4-a87ec11928](https://user-images.githubusercontent.com/23560242/196544426-0aa7462d-7d73-4a58-89f4-1f7136a406b9.gif)

---

### Convert Solidity to Interface

Convert Solidity file into an interface using the convert2interface (download the right version of the Solc then compile and extract the interface).

![image](https://user-images.githubusercontent.com/23560242/178570537-8974f67c-baa6-4e8d-b2e9-c4f8ad5ca9e5.png)

Just need to use the `--path` parameter.

---

### Mempool Watcher

A small Mempool Watcher written in Rust.

- Need to add lots of features to filter every txs.
- Next step add a features to simulates txs.

## ![image](https://user-images.githubusercontent.com/23560242/179367699-286e92ac-ce70-4f6e-9e20-434d8b565972.png)

### Contract-info.

Get informations using AST of the contract.
/!\ This doesn't compile the contract /!\
To have a quick overview of the (functions, modifiers, visibility, library etc...) you can use `-m` for **modifiers** and `-v` for the **visibility**.

_Usage :_
![image](https://user-images.githubusercontent.com/23560242/196519295-c9881b79-602d-43eb-bed8-bd8726750d3c.png)

**Centralisation Risk**

- The `-c` set to `yes` will return some crisk ready to share in markdown (need to pass the modifier).
  ![image](https://user-images.githubusercontent.com/23560242/197516359-611afd0f-c342-4ca2-87fd-adeb56b764a7.png)

_Usage :_
`swek --path /Users/ethnical/Sec/Rust/oz_implementations/contracts/OffShore.sol -m onlyOwner -c yes`

---

### Implemented Features

- Visibility Checker : Display all functions in _red_ who has `_` but not `internal` visibility.

### Upcoming Features.

_Contracts Features:_

- [X] Analyze directly in (etherscan, snowtrace, moonscan, etc).
- [ ] Add the `view` `pure` etc in modifiers.
- [ ] Add a params for remapping like `--remaps @openzeppelin/contracts/=lib/openzeppelin-contracts/contracts/`
- [ ] Display all the contracts names inside the folder.
- [ ] Display all the externals calls.

_Others features:_

- [X] private key to address.
- [ ] Converter Wei Gas.
- [ ] ByteCode ASM deploy.
- [ ] Get interface from non compile contract
- [ ] Check the dif between OZ implentation and the implentation inside the sol file.
- [ ] Reorganise inside a file the interface and the contract to have them in correct order.
- [ ] Create a markdown option to store as a markdown file.
- [ ] Add a kind of "binwalk" on calldata (4bytes signatures).
---

### TODO

- [ ] Reuse the Solang lib directly to get the `notice` through comments.
- [ ] Use the solang library
