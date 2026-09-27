# 開発方針・開発環境ルール(aruaru-jobs)

作業ドライブは`F:\runo`。この節は
[`open-raid-z`](https://github.com/aon-co-jp/open-raid-z)の`CLAUDE.md`を
正本とし、各プロジェクトへコピーして同期する既存の運用ルール継承方針に
準じる(全リポジトリ共通ルールはここに複製せず`open-raid-z/CLAUDE.md`を
参照すること)。

## このリポジトリの役割

フリーランス案件をまとめて検索できる集約サイト。検索自体は
`aruaru-llm`側の`/v1/search/raw`(Google検索経由、aruaru-search優先)へ
委譲し、このリポジトリ自身は(1)静的フロントエンド配信 (2)
`/api/jobs/search`での検索プロキシ、の2つだけを行う。

## 関連プロジェクト

- **aruaru-llm**(検索・AI推論エンジン本体): https://github.com/aon-co-jp/aruaru-llm
- **maid-cafe-programming-school**(VS Code拡張機能、`vscode://`リンク
  経由の連携先): https://github.com/aon-co-jp/maid-cafe-programming-school
- **open-english**(姉妹プロジェクト、AIプロバイダ選択・検索補強の
  設計の正本): https://github.com/aon-co-jp/open-english

## 実装状況(2026-09-28、初回スキャフォールド)

- `server/src/main.rs`: `open-runo-poem-compat`
  (`F:\runo\repository\RPoem\crates\open-runo-poem-compat`をpath依存)
  経由の最小サーバー。ルート: `GET /`・`GET /app.js`・`GET /style.css`・
  `POST /api/jobs/search`。CORS+gzip圧縮ミドルウェアを`.with_cors()`
  `.with_compression()`で適用。
- `web/`: 検索フォーム+結果一覧のみの最小フロントエンド。
  `Maid Cafe Programming School`側の`import-job`ハンドラが期待する
  クエリパラメータ名(`title`/`url`/`snippet`)と一致させてある——
  変更する場合は両リポジトリで同期すること。

## 正直な開示・既知の制約

- `aruaru-llm`側の実際のレスポンス形状(`results`フィールドの中身)は
  ソースを確認した上で`extractItems()`(`web/app.js`)で複数の妥当な
  形状を試す実装にしてあるが、実際に稼働中の`aruaru-llm`に対する
  E2E検証(実HTTP往復)はまだ実施していない——`cargo check`での
  コンパイル確認までに留まる。
- VPSへのデプロイ・systemdサービス化は本CLAUDE.mdの執筆時点では
  未実施(次にすべきこと参照)。

## 次にすべきこと

- 実際に稼働中の`aruaru-llm`インスタンスに対する`/api/jobs/search`の
  実HTTP E2E検証。
- VPS(`easy-web.tokyo`等)へのデプロイ(`open-english`の
  デプロイ手順——`open-web-server`の`domains.toml`へのpath_prefix
  追記、systemdサービス化——を参考にできる)。
- 検索結果のUI(現状は最小限、案件のカテゴリ・地域絞り込み等は未実装)。
