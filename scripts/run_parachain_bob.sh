./target/release/parachain-collator \
--bob \
--collator \
--force-authoring \
--chain res/realis-testnet-raw.json \
--base-path /tmp/parachain/bob \
--bootnodes /ip4/127.0.0.1/tcp/40333/p2p/12D3KooWJQ6qjaxYRrW9TeANQFXC6Qkx6TbL1fHdsF9zpnpMXUtR \
--port 40334 \
--ws-port 8845 \
-- \
--execution wasm \
--chain /home/user/CLionProjects/polkadot/res/realis-testnet-raw.json \
--port 30344 \
--ws-port 9978