# xconfig

Control the XMRig miner without having to open the JSON configuration file or any other expensive graphical interfaces.

## Demo

[![asciicast](https://asciinema.org/a/lH9tY1rlbZuRUV3q1LTbVYfVx.svg)](https://asciinema.org/a/lH9tY1rlbZuRUV3q1LTbVYfVx)



## Instalation
<!-- #### For an immediately usable binary version see -->

<!-- https://github.com/MoonDusk1996/xconfig/releases/new -->

#### From source

If you want to build xconfig from source, you need `Rust`. You can then use `cargo` to build everything:

```bash
  git clone https://github.com/MoonDusk1996/xconfig.git
  cd xconfig
  cargo build --release
  ./target/release/xconfig
```
### After installation
Be sure to specify the path to your XMRig configuration file in: `.config/xconfig/xconfig.json`

```
{
  "xmrig_config_file_path": "put/the/path/to/xmrig.json"
}
```
