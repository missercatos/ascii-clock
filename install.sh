#!/bin/bash
# ascii-clock 安装脚本
# 简单脚本
set -e

echo "编译中..."
cargo build --release

BIN="$(dirname "$0")/target/release/ascii_clock"
if [ ! -f "$BIN" ]; then
    echo "编译失败" >&2
    exit 1
fi

mkdir -p ~/.local/bin
cp "$BIN" ~/.local/bin/clock
chmod +x ~/.local/bin/clock
echo "已安装到 ~/.local/bin/clock"
echo "使用方式：终端输入 clock 即可"
