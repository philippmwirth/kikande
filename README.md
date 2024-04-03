<p align="center">
  <img src="docs/logo/kikande_logo.png" width=200 height=200/>
  
</p>
<h1 align="center"; style="font-size:18px; ">Kikande</h1>
<p align="center";"> Stockfish, but for Bao la Kiswahili.


## Usage

Install `rustup` (for instructions on Windows see [here](https://doc.rust-lang.org/cargo/getting-started/installation.html)):

```
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
```

### Build

```
cargo build --release
```

### Find the best move
```
cargo run --release search --depth 16 --threads 4
```


### Play against the computer
```
cargo run --release play --difficulty 3
```