#!/bin/bash

set -e  # エラーが発生した時点でスクリプトを終了

# 関数定義
log() {
    echo "[$(date '+%Y-%m-%d %H:%M:%S')] $1"
}

# 並列ビルドのためのCPUコア数を取得
CORES=$(nproc)

# Rustのデフォルトツールチェーンを設定（必要な場合のみ）
if [[ "$(rustc --version)" != *"stable"* ]]; then
    log "Rustのデフォルトツールチェーンを安定版に設定します..."
    rustup default stable
fi

# プロジェクトディレクトリに移動
log "プロジェクトディレクトリに移動します..."
cd ~/RustroverProjects/parallel-dos-attack-tool/ || { log "ディレクトリが見つかりません"; exit 1; }

# プロジェクトをビルド
log "プロジェクトを${CORES}個のコアでビルドします..."
RUSTFLAGS="-C target-cpu=native" cargo build --release -j"${CORES}" || { log "ビルドに失敗しました"; exit 1; }

# 実行ファイルに権限を付与
log "実行ファイルに権限を付与します..."
sudo setcap cap_net_raw,cap_net_admin=eip target/release/parallel-dos-attack-tool

log "アプリケーションを実行します..."
sudo ./target/release/parallel-dos-attack-tool