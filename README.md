# Multi-party ECDSA

[origin repo](https://github.com/ZenGo-X/multi-party-ecdsa)

Original implementation requires params.json file located in the executed folder.



```bash
# params.json 
{
  "parties": "1",
  "threshold": "2"
}
```
It brings complexity to run several gg18_sm_manager at the same time. To use different parameters in params.json file there are should be several located and executed in different folders.


To be able to run several servers in the same folder and use different params, current implementation get parties and threshold from env arguments. 

## Build 

For Linux:
```bash
cargo build --release --examples
```

For MacOs:
```bash
cargo build --release --examples --no-default-features --features curv-kzen/num-bigint
```
## Run GG18 Demo

The following steps are for setup, key generation with `n` parties and signing with `t+1` parties.

### Setup


```bash
./gg18_sm_manager <threshold> <parties>

# ./gg18_sm_manager 1 2 
```

To run server on specific port:
```bash
ROCKET_PORT=<port> ./gg18_sm_manager 1 2 
```

To give external access to the server:
```bash
ROCKET_ADDRESS=0.0.0.0 ./gg18_sm_manager 1 2 
```


### KeyGen

```bash
./gg18_keygen_client http://127.0.0.1:8000 <shares_filename> <threshold> <parties>

# ./gg18_keygen_client http://127.0.0.1:8000 keys.store 1 2 
```

### Sign


```bash
./gg18_sign_client http://127.0.0.1:8000 <shares_filename> <text_to_sign> <threshold> <parties>

# ./gg18_sign_client http://127.0.0.1:8000 keys.store "hello" 1 2 

```
