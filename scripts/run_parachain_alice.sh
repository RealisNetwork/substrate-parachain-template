./target/release/parachain-collator \
--alice \
--collator \
--force-authoring \
--chain res/realis-testnet-raw.json \
--base-path /tmp/parachain/alice \
--port 40333 \
--ws-port 8844 \
-- \
--execution wasm \
--chain /home/user/CLionProjects/polkadot/res/realis-testnet-raw.json \
--port 30343 \
--ws-port 9977