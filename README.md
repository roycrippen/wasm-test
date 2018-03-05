# wasm-test
wasm/rust testing


install
```
yarn
```

build and run
```
$ yarn run webpack
$ node dist/index.js
```

run rust bin
``` 
cargo run
or
cargo run --release

[roy@T5500 wasm-test]$ time cargo run --release
    Finished release [optimized] target(s) in 0.0 secs
     Running `target/release/wasm-test`
res = 617100

real    0m0.233s
user    0m0.203s
sys     0m0.027s
```



