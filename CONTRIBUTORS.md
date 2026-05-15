## Generating `snout.h`

generating snout.h requires `cbindgen`
This can either be installed from your package manager. Or from cargo:
```sh
cargo install --force cbindgen
export PATH=$PATH:$HOME/.cargo/bin
``` 

Once cbindgen is installed and located on your PATH, `snout.h` can be generated via:
```sh
cbindgen --config cbindgen.toml --output include/snout.h
```

The generated `snout.h` file will then be located under `include/snout.h`

