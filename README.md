# Libsnout

This is a rust implementation of Project Babble's baballonia face tracking sofware.
It's designed to be a library; easy to integrate in a variety of frontend projects.

## Building libsnout

Make sure you have `llvm-devel` installed.
A working face tracking model is supplied. It's the same as in the baballonia repository, but ran through `onnxsim`.

On fedora it's:
```sh
dnf install llvm llvm-devel onnxruntime onnxruntime-devel
```

## Building and running the CLI

Configure the `config.toml` to your liking and run. It will show the different commands.

```sh
cargo run --release -p snout-cli -- -c config.toml help
```

## Notes on cropping

cropping the image works slightly differently; instead of providing top/left/right/bottom coordinates it uses major/minor shift and scale.
Scale 1 is 100%, increase it to zoom in (1.5 would be 150%). Major and minor shift go from -1 to 1.

Major shifts along the longest axis, minor along the shortest axis. Minor shift only does something when zoomed in, if your input is a square then both will only function when zoomed in. It will always crop square; so on a 16:9 image the sides are trimmed off along the longest axis, for example. 
Major shift will then allow you to shift the crop left and right.

I designed it this way to prevent users for squishing their face, since the model always wants a 240x240 pixel input and the image pipeline just squishes the cropped image to fit that, squishing your face if you don't have a perfectly square crop.

### Generating `snout.h`

```sh
cargo install --force cbindgen

export PATH=$PATH:/home/proto/.cargo/bin
cbindgen --config cbindgen.toml --output include/snout.h
```

## License

Right now it's licensed under the same license as Baballonia from Project Babble is, considering this is a derivative work.
