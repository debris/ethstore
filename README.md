# ethstore

[![Build Status][travis-image]][travis-url]

[travis-image]: https://travis-ci.org/debris/ethstore.svg?branch=master
[travis-url]: https://travis-ci.org/debris/ethstore

Ethereum key management.

[Documentation](http://debris.github.io/ethstore/ethstore/index.html)

### Usage

```
Ethereum key management.
  Copyright 2016 Ethcore (UK) Limited

Usage:
    ethstore create dir <dir> (random | prefix <p> <i> | brain <seed>)
    ethstore create (geth | parity) (random | prefix <p> <i> | brain <seed>) [--testnet]
    ethstore change-pwd dir <dir> <address> <old-pwd> <new-pwd>
    ethstore change-pwd (geth | parity) <address> <old-pwd> <new-pwd> [--testnet]
    ethstore list dir <dir>
    ethstore list (geth | parity) [--testnet]
    ethstore export dir <src> <dst>
    ethstore export geth parity [--testnet]
    ethstore remove dir <dir> <address> <password>
    ethstore remove (geth | parity) <address> <password> [--testnet]
    ethstore sign dir <dir> <address> <password> <message>
    ethstore sign (geth | parity) <address> <password> <message> [--testnet]
    ethstore [-h | --help]

Options:
    -h, --help         Display this message and exit.
    --testnet          Use testnet secret store.

Commands:
    create             Create new account.
    change-pwd         Change password.
    list               List accounts.
    export             Export accounts src to dst.
    remove             Remove account.
    sign               Sign message.
    dir                Use keystore located in directory.
    parity             Use parity keystore.
    geth               Use geth keystore.
    random             Generate new account randomly.
    prefix             Generate new account with prefixed address.
    brain              Generate new brain-wallet account.
```
