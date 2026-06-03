#!/data/data/com.termux/files/usr/bin/bash
# ============================================================
#  training_mod 日本語版 — Termux 一括アップロードスクリプト
#  使い方:
#    chmod +x upload_to_github.sh
#    ./upload_to_github.sh
# ============================================================

set -euo pipefail

# ──────────────────────────────────────────────
# 設定 (必要に応じて編集してください)
# ──────────────────────────────────────────────
GITHUB_USER="tou108"
REPO_NAME="training_mod"
REMOTE_URL="https://github.com/${GITHUB_USER}/${REPO_NAME}.git"
BRANCH="main"
COMMIT_MSG="feat: UI日本語化 & GitHub Actions NROビルド追加"

# ──────────────────────────────────────────────
# 必要パッケージの確認・インストール
# ──────────────────────────────────────────────
echo "==> 依存パッケージを確認しています..."
pkg install -y git openssh 2>/dev/null || true

# ──────────────────────────────────────────────
# Git の初期設定（未設定の場合）
# ──────────────────────────────────────────────
if [ -z "$(git config --global user.email 2>/dev/null)" ]; then
  echo "==> Git のユーザー情報を設定します"
  read -p "GitHub メールアドレス: " GIT_EMAIL
  read -p "GitHub ユーザー名: " GIT_NAME
  git config --global user.email "$GIT_EMAIL"
  git config --global user.name  "$GIT_NAME"
fi

# ──────────────────────────────────────────────
# 認証ヘルパー: credential.helper に store を使用
# (初回のみGitHubのPersonal Access Tokenを入力)
# ──────────────────────────────────────────────
git config --global credential.helper store
if ! grep -q "github.com" ~/.git-credentials 2>/dev/null; then
  echo ""
  echo "==> GitHub Personal Access Token (PAT) が必要です"
  echo "    GitHub → Settings → Developer settings"
  echo "    → Personal access tokens → Tokens (classic)"
  echo "    → 'repo' スコープにチェックして生成してください"
  echo ""
  read -p "GitHub ユーザー名: " PAT_USER
  read -s -p "Personal Access Token: " PAT_TOKEN
  echo ""
  echo "https://${PAT_USER}:${PAT_TOKEN}@github.com" >> ~/.git-credentials
  chmod 600 ~/.git-credentials
fi

# ──────────────────────────────────────────────
# リポジトリの初期化 or 既存リポジトリの更新
# ──────────────────────────────────────────────
WORK_DIR="$HOME/training_mod"

if [ -d "$WORK_DIR/.git" ]; then
  echo "==> 既存リポジトリを検出 — git pull で最新化..."
  cd "$WORK_DIR"
  git pull origin "$BRANCH" --rebase || true
else
  echo "==> リポジトリが見つかりません"
  echo "    ソースコードをこのスクリプトと同じディレクトリに置いてください"
  echo "    期待するパス: $WORK_DIR"
  echo ""
  echo "    例: PC で zip を展開して adb push か scp で転送後、"
  echo "    mv ~/storage/downloads/training_mod $WORK_DIR"
  echo "    を実行してから再度このスクリプトを実行してください。"
  exit 1
fi

# ──────────────────────────────────────────────
# remote の確認 & 設定
# ──────────────────────────────────────────────
cd "$WORK_DIR"

if git remote get-url origin &>/dev/null; then
  echo "==> remote 'origin' は既に設定されています: $(git remote get-url origin)"
else
  echo "==> remote 'origin' を設定します: $REMOTE_URL"
  git remote add origin "$REMOTE_URL"
fi

# ──────────────────────────────────────────────
# ステージング & コミット
# ──────────────────────────────────────────────
echo "==> 変更をステージングしています..."
git add -A

if git diff --cached --quiet; then
  echo "==> コミットする変更がありません。最新の状態です。"
  exit 0
fi

echo "==> コミット中: $COMMIT_MSG"
git commit -m "$COMMIT_MSG"

# ──────────────────────────────────────────────
# プッシュ
# ──────────────────────────────────────────────
echo "==> GitHub へプッシュしています..."
git push -u origin "$BRANCH"

echo ""
echo "============================================"
echo "  アップロード完了！"
echo "  https://github.com/${GITHUB_USER}/${REPO_NAME}"
echo ""
echo "  GitHub Actions のビルド状況:"
echo "  https://github.com/${GITHUB_USER}/${REPO_NAME}/actions"
echo "============================================"
