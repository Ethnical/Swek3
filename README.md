# Swek3

## A toolbox to pentest web3 written in Rust.

![ezgif com-gif-maker](https://user-images.githubusercontent.com/23560242/190014998-f2efbac7-3991-448e-89af-efcc0721f9f2.gif)

### Upcoming Features.

_Contracts Features:_

- [ ] Add a params for remapping like `--remaps @openzeppelin/contracts/=lib/openzeppelin-contracts/contracts/`
- [ ] Display all the contracts names inside the folder.
- [ ] Display if function has `_` but not internal.

_Others features:_

- [ ] Converter Wei Gas.
- [ ] ByteCode ASM deploy.
- [ ] Get interface from non compile contract
- [ ] Check the dif between OZ implentation and the implentation inside the sol file.
- [ ] Reorganise inside a file the interface and the contract to have them in correct order.

### Convert Solidity to Interface

Convert Solidity file into an interface using the convert2interface (download the right version of the Solc then compile and extract the interface).

![image](https://user-images.githubusercontent.com/23560242/178570537-8974f67c-baa6-4e8d-b2e9-c4f8ad5ca9e5.png)

Just need to use the `--path` parameter.

### Mempool Watcher

A small Mempool Watcher in Rust.

- Need to add lots of features to filter every txs.
- Next step add a features to simulates txs.

![image](https://user-images.githubusercontent.com/23560242/179367699-286e92ac-ce70-4f6e-9e20-434d8b565972.png)

### Centralisation Risk + Modifiers

Using the paramaters `--modifier` you could specify the name of the modifier you want.

- Crisk
  <img width="1317" alt="image" src="https://user-images.githubusercontent.com/23560242/190011679-2665d5d0-4ec9-4859-96a0-31d03d0adde2.png">

_Usage :_
`swek modifiers --path /Users/ethnical/Sec/Rust/oz_implementations/contracts/OffShore.sol -m onlyOwner -c true`

This will export the Crisk has markdown.

- Modifiers
  Using `--modifier` will create a filter on the selected modifier.
  <img width="574" alt="image" src="https://user-images.githubusercontent.com/23560242/190029156-a83fad48-c2be-485a-b5f5-7ce3f2d6305b.png">

---
