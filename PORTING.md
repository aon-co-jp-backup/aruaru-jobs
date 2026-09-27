# PORTING(お引越し可能ファイル一覧) — 日本語 / English

## 1. `open-runo-poem-compat`最小サーバーの雛形

`server/src/main.rs`は、RPoemの`open-runo-poem-compat`ファサード
(`Route::new().at(path, get(h).post(h2)).with_cors().with_compression()`
+`Server::new(TcpListener::bind(addr)).run(app)`)を使った、静的配信+
1本のJSON APIプロキシという最小構成のサーバー実装例。新規に軽量な
Rustサーバーを別プロジェクトで起こす際の雛形としてそのまま使える。

`server/src/main.rs` is a minimal example of the `open-runo-poem-compat`
facade (`Route::new().at(path, get(h).post(h2)).with_cors()
.with_compression()` + `Server::new(TcpListener::bind(addr)).run(app)`)
serving static files plus one JSON API proxy. It can be reused directly
as a template for a new lightweight Rust server in another project.

## 2. aruaru-llm検索プロキシの呼び出しパターン

`job_search_handler`(`server/src/main.rs`)が示す
`{"source": "google", "query": ..., "max_results": N, "free_only": true}`
というペイロード形状は、`aruaru-llm`の`/v1/search/raw`を呼ぶ他の
どのプロジェクトでも同じ形で再利用できる。

The payload shape in `job_search_handler`
(`{"source": "google", "query": ..., "max_results": N, "free_only":
true}`) is the same shape any other project should use when calling
`aruaru-llm`'s `/v1/search/raw`.

## 3. `vscode://`ディープリンク生成パターン

`web/app.js`の`buildVscodeImportUrl`は、`URLSearchParams`で
`title`/`url`/`snippet`をエンコードして`vscode://<publisher>.
<extension-id>/<path>?<query>`を組み立てるだけの単純な関数。
外部Webサイトから任意のVS Code拡張機能へディープリンクを張りたい場合の
最小実装として移植できる。

`buildVscodeImportUrl` in `web/app.js` is a simple function that encodes
`title`/`url`/`snippet` via `URLSearchParams` into a
`vscode://<publisher>.<extension-id>/<path>?<query>` link. It's a
minimal, portable pattern for linking from any external website into a
VS Code extension.
