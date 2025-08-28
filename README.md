# Ethereum Local Node Setup

This repository contains instructions for setting up a **local Ethereum development node** using Geth (`geth`) and Docker Compose, creating accounts, sending transactions, and Git setup using SSH.

---

## Table of Contents

* [Requirements](#requirements)
* [Docker Compose Setup](#docker-compose-setup)
* [Running the Node](#running-the-node)
* [Attaching to Geth Console](#attaching-to-geth-console)
* [Creating Accounts](#creating-accounts)
* [Sending Transactions](#sending-transactions)
* [Git Setup Using SSH](#git-setup-using-ssh)

---

## Requirements

* Docker & Docker Compose installed
* Basic knowledge of Ethereum & Geth
* Git installed and configured
* GitHub account with SSH key added

---

## Docker Compose Setup

Create a `docker-compose.yml` file with the following content:

```yaml
version: "3.8"
services:
  ethereum-node:
    image: ethereum/client-go:stable
    container_name: ethereum-node
    ports:
      - "8545:8545"
      - "30303:30303"
    command: >
      geth --dev \
           --http \
           --http.addr 0.0.0.0 \
           --http.api admin,eth,net,web3,miner,txpool,personal \
           console
```

* `--dev` mode creates an ephemeral development chain with a pre-funded dev account.
* Ports `8545` (HTTP RPC) and `30303` (P2P) are exposed.

---

## Running the Node

```bash
docker compose up
```

* The container will start and run Geth in dev mode.
* Blocks are mined automatically on demand.
* The container stays running while Geth console is active.

---

## Attaching to Geth Console

### Using IPC:

```bash
docker exec -it ethereum-node geth attach ipc:/tmp/geth.ipc
```

### Using HTTP:

```bash
docker exec -it ethereum-node geth attach http://localhost:8545
```

### Check accounts and balances:

```javascript
eth.accounts
eth.getBalance(eth.coinbase)
```

---

## Creating Accounts

```javascript
personal.newAccount("yourStrongPassword")
```

* This will create a new Ethereum account.
* View all accounts:

```javascript
eth.accounts
```

---

## Sending Transactions

```javascript
eth.sendTransaction({
  from: eth.accounts[0],
  to: eth.accounts[1],
  value: web3.toWei(10, "ether")
})
```

* In `--dev` mode, transactions are mined immediately.
* Check balances after transaction:

```javascript
eth.getBalance(eth.accounts[0])
eth.getBalance(eth.accounts[1])
```

---

## Git Setup Using SSH

1. **Change remote URL to SSH:**

```bash
git remote set-url origin git@github.com:manaslaud/eth-node.git
```

2. **Verify remote:**

```bash
git remote -v
```

3. **Push to GitHub:**

```bash
git push -u origin main
```

> Make sure your SSH key is added to GitHub:
> `cat ~/.ssh/id_rsa.pub` → Copy to **GitHub → Settings → SSH and GPG keys**

---

## Notes

* `--dev` mode is for testing only; the chain and accounts are ephemeral.
* Use HTTP or IPC to interact with Geth.
* You can create multiple accounts and send transactions freely.
* Blocks are mined automatically when there are pending transactions.

---

## References

* [Geth Documentation](https://geth.ethereum.org/docs/)
* [Docker Hub: ethereum/client-go](https://hub.docker.com/r/ethereum/client-go)
* [Ethereum JavaScript Console API](https://geth.ethereum.org/docs/rpc/server)
