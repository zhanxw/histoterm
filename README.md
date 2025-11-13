# histoterm

![Build](https://img.shields.io/badge/build-passing-brightgreen)
![License: GPLv3]([https://img.shields.io/badge/License-MIT-blue](https://img.shields.io/badge/License-GPLv3-blue))
![Rust](https://img.shields.io/badge/Rust-stable-orange)

A lightweight Rust CLI tool that reads numeric data from **stdin** and prints an **ASCII histogram** showing the data distribution — perfect for quick data exploration right from the terminal.

---

## 🧩 Installation

### Option 1 — From local source

```bash
cargo install --path .
```

This installs `histoterm` into Cargo’s bin directory (usually `~/.cargo/bin`).

---

### Option 2 — Directly from GitHub

Install the latest version straight from the repository:

```bash
cargo install --git https://github.com/zhanxw/histoterm
```

Or specify a branch, tag, or commit:

```bash
cargo install --git https://github.com/zhanxw/histoterm --branch main
```

---

## 🛠️ Build from source

If you just want to build without installing:

```bash
cargo build --release
```

The compiled binary will be at:
```
target/release/histoterm
```

---

## 🖼️ Preview

Here’s what `histoterm` looks like in action:

```text
$ seq 100 | awk 'BEGIN{srand()} {for(i=0;i<int(rand()*3);i++) print $1}'   | histoterm --bins 12 --width 60

ASCII Histogram  (bins=12, width=60)
count = 86, min = 3.000000, max = 100.000000, mean = 47.639535, std = 29.028419

[3.0000     ,    11.0833) |     14 ( 16.3%) | ############################################################
[11.0833    ,    19.1667) |      4 (  4.7%) | #################
[19.1667    ,    27.2500) |     10 ( 11.6%) | ##########################################
[27.2500    ,    35.3333) |      7 (  8.1%) | ##############################
[35.3333    ,    43.4167) |      6 (  7.0%) | #########################
[43.4167    ,    51.5000) |      5 (  5.8%) | #####################
[51.5000    ,    59.5833) |      6 (  7.0%) | #########################
[59.5833    ,    67.6667) |     10 ( 11.6%) | ##########################################
[67.6667    ,    75.7500) |      4 (  4.7%) | #################
[75.7500    ,    83.8333) |      9 ( 10.5%) | ######################################
[83.8333    ,    91.9167) |      4 (  4.7%) | #################
[91.9167    ,   100.0000) |      7 (  8.1%) | ##############################
```

---

## ⚙️ Command-line options

| Option | Description | Default |
|--------|--------------|----------|
| `--bins <N>` | Number of histogram bins | `10` |
| `--width <W>` | Maximum bar width (characters) | `50` |

---

## 🧮 Usage examples and piping

You can pipe **any stream of numbers** into `histoterm`.  
Here are some practical examples:

### 1️⃣ From a CSV column
Use `awk`, `cut`, or `csvkit` to extract numeric data:

```bash
cut -d, -f3 data.csv | tail -n +2 | histoterm --bins 20
```

For tab-separated files:
```bash
awk -F'\t' '{print $5}' data.tsv | histoterm
```

---

### 2️⃣ From logs or JSON data
Extract numeric values from logs:
```bash
grep "latency=" server.log | sed 's/.*latency=//' | histoterm
```

From JSON data:
```bash
jq '.response_time' data.json | histoterm --bins 15
```

---

### 3️⃣ Combine with other tools
Chain with statistical or simulation outputs:
```bash
python -c "import numpy as np; [print(x) for x in np.random.normal(0,1,1000)]"   | histoterm --bins 20 --width 70
```

---

## 📄 License

GPLv3 License © 2025

This program is free software: you can redistribute it and/or modify
it under the terms of the GNU General Public License as published by
the Free Software Foundation, either version 3 of the License, or (at your option) any later version.

This program is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU General Public License for more details.

You should have received a copy of the GNU General Public License
along with this program.  If not, see <https://www.gnu.org/licenses/>.
