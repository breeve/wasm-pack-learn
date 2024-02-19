# tools
## version
wasm-pack: 0.12.1
npm: 8.5.1
wasm-bindgen: 0.2.84
bevy: 0.13.0
webpack: 4.29.3

# steps
## create wasm project
1. wasm-pack new my-game

## build & link local
1. cd my-game; wasm-pack build --out-dir pkg-bundler --target bundler
2. cd pkg-bundler; npm link; cd ..

## create http project & link local 'my-game'
1. npm init wasm-app www-wasm-app; 
2. cd www-wasm-app; npm link my-game;

## start
1. cd www-wasm-app;
2. npm run start -- --port 9000 --host 192.168.95.33

# build option
`export RUST_LOG=info` enable build log, like these
```
RUST_LOG=info wasm-pack build --out-dir pkg-bundler --target bundler
```
valid values: `off,error,warn,info,debug`
