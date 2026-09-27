# aruaru-jobs

フリーランス案件をまとめて検索できる集約サイト。検索自体は
[`aruaru-llm`](https://github.com/aon-co-jp/aruaru-llm)経由の
**aruaru-search**(無制限・APIキー不要)へ委譲し、このサーバー自身は
静的フロントエンド配信+検索プロキシのみを行う。各案件から
[`Maid Cafe Programming School`](https://github.com/aon-co-jp/maid-cafe-programming-school)
(VS Code拡張機能)へ`vscode://`リンクで直接開発を始められ、案件情報を
種にプログラミングを学習しながら一緒に開発できる。

## 構成

- `server/`(`aruaru-jobs-server`、Rust): `open-runo-poem-compat`
  (RPoemのhyperベースサーバーをpoem風APIでラップしたファサード)を
  使った薄いHTTPサーバー。`web/`配下の静的ファイルを配信し、
  `POST /api/jobs/search`で`aruaru-llm`の`/v1/search/raw`
  (`source: "google"`、aruaru-search優先・無料枠は自信が無い時のみ)へ
  プロキシする。
- `web/`: `index.html`/`style.css`/`app.js`。案件検索フォーム+
  結果一覧+各案件の「🎓 Maid Cafe Programming Schoolで開発」ボタン
  (`vscode://aon-co-jp.maid-cafe-programming-school/import-job?
  title=...&url=...&snippet=...`)。

## セットアップ

```bash
cd server
ARUARU_LLM_BASE_URL=http://127.0.0.1:4600 \
ARUARU_JOBS_BIND_ADDR=0.0.0.0:4700 \
cargo run --release
```

`aruaru-llm`が起動していない場合、検索は正直に接続エラーを返す
(存在しない結果を捏造しない)。

## 正直な開示

- このサイト自身は求人サイトのスクレイピング・独自クローラーを持たない
  ——検索は常に`aruaru-llm`側のGoogle検索経由の`/v1/search/raw`に
  委譲する。
- `Maid Cafe Programming School`ボタンは同名のVS Code拡張機能が
  インストールされていない環境では機能しない(`vscode://`リンクが
  素通りするだけ)。

## Honest disclosure (English)

This site has no scraper or crawler of its own — all search is
delegated to `aruaru-llm`'s Google-backed `/v1/search/raw` (aruaru-search
first, free-tier Google fallback only when the AI is unsure). The
"develop with Maid Cafe Programming School" button only works if that
VS Code extension is installed; otherwise the `vscode://` link is simply
a no-op.
