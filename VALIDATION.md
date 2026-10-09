# Validation

2026-10-09 / Windows x64 / Rust 1.96.0 MSVC / BoxLinkBox 1.0.0

- cargo fmt --check：成功
- Clippy --all-targets -D warnings：成功
- Rustテスト12件：成功（URL検証、上限、SQLite復旧、原子的保存、バックアップ、取り消し履歴、差分状態送信）
- Release EXEの実GUI確認：起動時にグループ0件、グループ・URLの作成/変更/削除、検索、取り消し、2つの実WebView、タブ切替、戻る/進む/再読込、無効URL、テーマ、再起動後の登録データ・Cookie保持、重複タブ防止、MIT/第三者通知/プライバシー/利用説明の表示を確認
- 最小画面860×600、倍率100/125/150/175/200%で管理画面の横はみ出しなし
- EXEの製品名・版番号・ファイル名：BoxLinkBox / 1.0.0 / BoxLinkBox.exe
- 専用データ環境で実施。Store版のデータはテストに使用していない
- Windows依存ライセンス98件：未設定・通知欠落0件
- RustSec監査：検出脆弱性0件。Cargo.lock全体にproc-macro-error 1.0.4の保守終了、glib 0.18.5のunsound警告あり。どちらもWindows x64の通常・ビルド依存グラフに含まれない。DB commit 550efd3d587a29b2e2c2b21b17a440da4fede999（2026-10-08更新）

この確認はあらゆるサイトの認証やWindows環境の動作を保証しません。EXEは署名なしです。GitHub Actionsはファイルを用意していますが、公開リポジトリでの実行結果は未確認です。
