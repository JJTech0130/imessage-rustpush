## imessage-rustpush

### Development
As you might need to make changes to upstream rustpush at the same time, here is the recommended way to set up your development environment:
```bash
git clone https://github.com/JJTech0130/imessage-rustpush.git
git clone https://github.com/OpenBubbles/rustpush.git
```

Then, you can open `imessage-rustpush.code-workspace` in VSCode and it should have both projects loaded.

In order to make cargo use your local copy of rustpush, you can add the following patch section to `Cargo.toml`:
```toml
[patch."https://github.com/OpenBubbles/rustpush"]
rustpush = { path = "../../../rustpush" }
```

### Building
```bash
./build.sh
```

It should produce a `imessage-rustpush` binary in the directory.

You will need Go and Rust installed already, instructions to get those installed might be added at some point.

### Running
First, you need to create a config file. Assuming you already have `bbctl` installed and do not already have a `sh-imessage` bridge registered, you can run:
```bash
bbctl --env staging c --type bridgev2 sh-imessage
```

Simply write that config to config.yaml in the same directory as the binary.

Then, you can run the binary:
```bash
./imessage-rustpush
```
