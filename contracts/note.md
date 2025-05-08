# Counter

## 实操

### 部署合约

```bash
rust-chain/contracts on  master [+?] is 📦 0.1.0 via 🦀 1.86.0 
➜ forge install                                 
Updating dependencies in /Users/qiaopengjun/Code/Rust/rust-chain/contracts/lib

rust-chain/contracts on  master [+?] is 📦 0.1.0 via 🦀 1.86.0 
➜ forge remappings > remappings.txt

rust-chain/contracts on  master [+?] is 📦 0.1.0 via 🦀 1.86.0 
➜ forge build                      
[⠊] Compiling...
No files changed, compilation skipped

rust-chain/contracts on  master [+?] is 📦 0.1.0 via 🦀 1.86.0 
➜ source .env           

rust-chain/contracts on  master [+?] is 📦 0.1.0 via 🦀 1.86.0 
➜ forge script CounterScript --rpc-url $HOLESKY_RPC_URL --broadcast --verify -vvvvv 
[⠊] Compiling...
No files changed, compilation skipped
Traces:
  [121] CounterScript::setUp()
    └─ ← [Stop]

  [167052] CounterScript::run()
    ├─ [0] VM::envUint("PRIVATE_KEY") [staticcall]
    │   └─ ← [Return] <env var value>
    ├─ [0] VM::startBroadcast(<pk>)
    │   └─ ← [Return]
    ├─ [96345] → new Counter@0x5A31b9407095dd9A295139c4F0183c076051632D
    │   └─ ← [Return] 481 bytes of code
    ├─ [0] console::log("Counter address: ", Counter: [0x5A31b9407095dd9A295139c4F0183c076051632D]) [staticcall]
    │   └─ ← [Stop]
    ├─ [22492] Counter::setNumber(69420 [6.942e4])
    │   └─ ← [Stop]
    ├─ [424] Counter::number() [staticcall]
    │   └─ ← [Return] 69420 [6.942e4]
    ├─ [0] console::log("Counter number: ", 69420 [6.942e4]) [staticcall]
    │   └─ ← [Stop]
    ├─ [0] VM::stopBroadcast()
    │   └─ ← [Return]
    └─ ← [Stop]


Script ran successfully.

== Logs ==
  Counter address:  0x5A31b9407095dd9A295139c4F0183c076051632D
  Counter number:  69420

## Setting up 1 EVM.
==========================
Simulated On-chain Traces:

  [96345] → new Counter@0x5A31b9407095dd9A295139c4F0183c076051632D
    └─ ← [Return] 481 bytes of code

  [22492] Counter::setNumber(69420 [6.942e4])
    └─ ← [Stop]


==========================

Chain 17000

Estimated gas price: 0.00349614 gwei

Estimated total gas used for script: 264249

Estimated amount required: 0.00000092385149886 ETH

==========================

##### holesky
✅  [Success] Hash: 0xc29b0a07ed4e464222824cf483c5fcb49d2be8a6d4b7928e69f3e322cf045c5d
Contract Address: 0x5A31b9407095dd9A295139c4F0183c076051632D
Block: 3787922
Paid: 0.000000548248384151 ETH (156817 gas * 0.003496103 gwei)


##### holesky
✅  [Success] Hash: 0x6bff519cb35a59975c8c7984d459e3651f7376fe33c68d15cecffd233e9489a4
Block: 3787922
Paid: 0.00000015284962316 ETH (43720 gas * 0.003496103 gwei)

✅ Sequence #1 on holesky | Total Paid: 0.000000701098007311 ETH (200537 gas * avg 0.003496103 gwei)
                                                                                                                                                                                                              

==========================

ONCHAIN EXECUTION COMPLETE & SUCCESSFUL.
##
Start verification for (1) contracts
Start verifying contract `0x5A31b9407095dd9A295139c4F0183c076051632D` deployed on holesky
EVM version: shanghai
Compiler version: 0.8.20

Submitting verification for [src/Counter.sol:Counter] 0x5A31b9407095dd9A295139c4F0183c076051632D.
Warning: Could not detect the deployment.; waiting 5 seconds before trying again (4 tries remaining)

Submitting verification for [src/Counter.sol:Counter] 0x5A31b9407095dd9A295139c4F0183c076051632D.
Warning: Could not detect the deployment.; waiting 5 seconds before trying again (3 tries remaining)

Submitting verification for [src/Counter.sol:Counter] 0x5A31b9407095dd9A295139c4F0183c076051632D.
Warning: Could not detect the deployment.; waiting 5 seconds before trying again (2 tries remaining)

Submitting verification for [src/Counter.sol:Counter] 0x5A31b9407095dd9A295139c4F0183c076051632D.
Warning: Could not detect the deployment.; waiting 5 seconds before trying again (1 tries remaining)

Submitting verification for [src/Counter.sol:Counter] 0x5A31b9407095dd9A295139c4F0183c076051632D.
Warning: Could not detect the deployment.; waiting 5 seconds before trying again (0 tries remaining)

Submitting verification for [src/Counter.sol:Counter] 0x5A31b9407095dd9A295139c4F0183c076051632D.
Submitted contract for verification:
        Response: `OK`
        GUID: `fgkwdamvcuihvdydrfaxyfwwnmwuwcncpri6ljnjtbwgaxajaw`
        URL: https://holesky.etherscan.io/address/0x5a31b9407095dd9a295139c4f0183c076051632d
Contract verification status:
Response: `NOTOK`
Details: `Pending in queue`
Warning: Verification is still pending...; waiting 15 seconds before trying again (7 tries remaining)
Contract verification status:
Response: `OK`
Details: `Pass - Verified`
Contract successfully verified
All (1) contracts were verified!

Transactions saved to: /Users/qiaopengjun/Code/Rust/rust-chain/contracts/broadcast/Counter.s.sol/17000/run-latest.json

Sensitive values saved to: /Users/qiaopengjun/Code/Rust/rust-chain/contracts/cache/Counter.s.sol/17000/run-latest.json

```

## 参考

- <https://holesky.etherscan.io/address/0x5a31b9407095dd9a295139c4f0183c076051632d>
