# bale

## File structure
<!--->
https://tree.nathanfriend.io/
[packagename].tar.zst
  bale.toml
  install.sh
  uninstall.sh
  data/
    whatever
<-->
```
.
└── [packagename].tar.zst/
    ├── bale.toml
    ├── install.sh
    ├── uninstall.sh
    └── data/
        └── whatever
```

## Compression
bales are compressed using `zstd` and tar, the resulting file is a `.tar.zst` file.
totally didnt steal this idea from pacman

## bale.toml
```toml
[package]
name = "bale-example"
version = "0.1.0"
description = "an example package"
deps = ["bale-example2"]
optional-deps = [
    { name = "bale-example3", description = "bale example 3" },
    { name = "bale-example4", description = "bale example 4" }
]
```
