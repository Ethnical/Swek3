# Swek3

## A web3 Security Toolbox written in Rust.



![ezgif-4-a87ec11928](https://user-images.githubusercontent.com/23560242/196544426-0aa7462d-7d73-4a58-89f4-1f7136a406b9.gif)




### Upcoming Features.

_Contracts Features:_

- [ ] Add a params for remapping like `--remaps @openzeppelin/contracts/=lib/openzeppelin-contracts/contracts/`
- [ ] Display all the contracts names inside the folder.
- [ ] Display all functions who has `_` but not `internal` flag.
- [ ] Display all the externals calls.

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

###  Get informations using `contract-info`.
/!\ This doesn't compile the contract /!\
To have a quick overview of the (functions, modifiers, visibility, library etc...) you can use `-m` for **modifiers** and `-v` for the **visibility**.

_Usage :_
![image](https://user-images.githubusercontent.com/23560242/196519295-c9881b79-602d-43eb-bed8-bd8726750d3c.png)

#### Centralisation Risk
- The `-c` set to `true` will return some crisk ready to share in markdown (need to pass the modifier).
  <img width="1317" alt="image" src="https://user-images.githubusercontent.com/23560242/190011679-2665d5d0-4ec9-4859-96a0-31d03d0adde2.png">

_Usage :_
`swek modifiers --path /Users/ethnical/Sec/Rust/oz_implementations/contracts/OffShore.sol -m onlyOwner -c true`


