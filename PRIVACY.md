# BoxLinkBox プライバシーポリシー

2026年10月9日 / バージョン1.0 / 提供者：MushApp

## 端末内の情報
グループ名、URL、名前、メモ、順序、テーマをSQLiteに保存します。取り消し履歴はメモリ内、直前の復旧用データはSQLiteに保持します。タブ一覧は実行中に保持します。URLのクエリやメモに秘密情報を含めないでください。

Cookie、LocalStorage、IndexedDB、キャッシュ等はWebView2専用プロファイルに保存され、サイトのログイン状態保持に使われます。独立したパスワード管理機能はありません。JSONバックアップにCookieやログイン状態は含みません。バックアップは利用者が指定した場所に保存します。

通常の保存先は `%LOCALAPPDATA%\BoxLinkBox\data` です。開発用BOXLINKBOX_DATA_DIRを指定した場合はその場所を使います。実際の使用パスは設定画面で確認できます。library.db、関連WAL/SHM、browser-profile、interface-profile、instance.lock、app.log等を作成します。登録データ・バックアップにアプリ独自の暗号化は施しません。ブラウザデータの保護はWindowsとWebView2に依存します。

アプリ独自のログはイベントコードに限定し、URL、メモ、Cookie、パスワードを記録しません。64KiBを超えると更新時にローテーションします。取り消し履歴は最大30操作かつJSON換算32MiBで、終了時に消えます。登録データとバックアップに自動削除期限はありません。

## 通信
本アプリは広告、解析、クラッシュ自動送信、開発者へのデータ送信、クラウド同期を実装していません。ページを開くと、そのサイトと関連サービスにIPアドレス、要求URL、ブラウザ情報、Cookie、フォーム入力、選択したアップロードファイル等が送信され得ます。HTTP通信は暗号化されない場合があります。カメラ、マイク、位置情報等のサイト権限は表示された許可内容を確認してください。タブ切替や最小化だけではページ通信の停止は保証されません。

閲覧先のデータ取扱いは各サイトの説明に従います。Microsoft Edge WebView2 Runtimeの更新・診断・データ処理は[Microsoftの説明](https://learn.microsoft.com/en-us/microsoft-edge/webview2/concepts/data-privacy)と[プライバシーステートメント](https://privacy.microsoft.com/privacystatement)を確認してください。共有Evergreen Runtimeを使い、Runtime本体は同梱しません。

Microsoft Defender SmartScreenを有効な既定状態で使用します。危険なサイト等の判定に必要な利用者情報がMicrosoftに送信される場合があります。上記Microsoftの説明をご確認ください。

GitHubでのダウンロード、Issue、Pull Request等はGitHubのサービス上の操作です。[GitHubのプライバシーステートメント](https://docs.github.com/en/site-policy/privacy-policies/github-general-privacy-statement)が適用されます。公開IssueやPull Requestには秘密情報を投稿しないでください。アプリ内の規約とライセンスの表示自体はローカルで完結します。

## 削除・問い合わせ
URL削除はCookie削除やログアウトではありません。サイトでログアウトするか、アプリ終了後にbrowser-profileを削除してください。登録データ等すべてを消す場合は、アプリ終了後に保存先全体を削除し、バックアップや外部同期先を別途削除してください。削除直後は取り消しや復旧用レコードに登録内容が残る場合があります。媒体からの復元不可能な消去は保証しません。開発者は端末内データを遠隔削除・復元できません。

問い合わせ窓口：mushapp.com@gmail.com。メールで送る情報は問い合わせ対応に使用します。パスワード、Cookie、トークン、秘密URL、他人の個人情報を添付しないでください。Gmailで受領するためGoogleの条件も適用されます。ブラウザプロファイル全体を送らないでください。

機能と情報の取扱いを変える場合は、この文書と版番号を更新します。改変版では、その提供者が実際の実装に合わせてこの説明を更新してください。
