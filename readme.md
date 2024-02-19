# tools
## version
xxx

# command
## new
`wasm-pack new <name> --template <template> --mode <normal|noinstall|force>`
default template is `rustwasm/wasm-pack-template`
default mode is `normal`, `noinstall` means that wasm-pack should not attempt to install any underlying tools. If a necessary tool cannot be found, the command will error. `force` means that wasm-pack should not check the local Rust version. If a local Rust is an unacceptable Rust version, the command will error.




# steps
## create wasm project
1. wasm-pack new my-game

## build & link local
1. cd my-game; wasm-pack build --out-dir pkg-bundler --target bundler
2. cd pkg-bundler; npm link; cd ..

## create http project
1. npm init wasm-app www-wasm-app; cd www-wasm-app; 
2. npm link my-game;
3. npm install --save @wasm-tool/wasm-pack-plugin
4. npm run start -- --port 9000 --host 192.168.95.33

