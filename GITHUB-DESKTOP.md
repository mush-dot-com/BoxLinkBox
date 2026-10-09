# GitHub Desktopで公開する

## ソースの送信
1. GitHub Desktopにmush-dot-comでログインします。
2. File → Add local repositoryを選び、このBoxLinkBoxフォルダを指定します。隣のWindows版フォルダではありません。
3. ローカルmainブランチには公開用ファイルをコミット済みです。Remoteは https://github.com/mush-dot-com/BoxLinkBox.git に設定しています。
4. Publish branch または Push originを押します。リポジトリは空の公開先として作成済みなので、同名のリポジトリを新規作成する必要はありません。
5. GitHubでREADME、LICENSE、src、ui、assets、tests、.github/workflowsが表示されていることを確認してください。ActionsのWindows build結果も確認できます。

ソースにはEXE、ユーザーデータ、ブラウザプロファイル、秘密鍵、Storeの識別設定を含めていません。コミットの名前はMushApp、メールは公開問い合わせ窓口です。別のGit作者情報を使いたい場合はGitHub Desktopの設定に合わせて変更してください。

## Windows版の配布
GitHubのReleases → Draft a new releaseで、タグ `v1.0.0`、タイトル `BoxLinkBox 1.0.0` とします。用意したWindows x64 ZIPとSHA256SUMS.txtを添付し、RELEASE-NOTES.mdの内容を説明に使って公開してください。EXEをsrcなどへコミットする必要はありません。

ソースZIPは.gitを含まない別配布物です。GitHub Desktopで送る場合は、ZIPを展開し直すより、上記のコミット済みBoxLinkBoxフォルダをそのまま指定してください。
