# オープンソース・商用配布の確認

確認日：2026-10-09 / BoxLinkBox 1.0.0 / Windows x64 / 同梱Cargo.lock

独自コード・図形素材はMIT Licenseで公開します。著作権・許諾表示を保持した改変、再配布、商用利用を認めます。第三者コードには各ライセンスが適用され、本体のMIT Licenseで上書きしません。

Windows x64の通常依存は92件、ビルド依存も含め98件です。固定版の原文を確認した範囲で、未設定ライセンス0件、原文欠落0件、GPL/AGPL/MPL指定0件。MIT、Apache-2.0、Unicode-3.0、Zlib等、通知・条件を保持して商用配布できる構成です。全件はLICENSE-INVENTORY.csv、原文はTHIRD-PARTY-NOTICES.txtを参照してください。ORは選択、ANDは両方の条件を意味します。

Tao 0.37.1はApache-2.0単独です。Wry 0.57.0はApache-2.0 OR MIT、rusqlite 0.40.2はMITです。SQLite本体はPublic Domainです。Rust 1.96.0標準ライブラリの通知はRUST-STDLIB-NOTICES.txtと元のHTMLに保持しています。

Microsoftの静的WebView2ローダーはSDK 1.0.3800.47のMicrosoft BSD 3-Clause形式のLICENSEとNOTICEを通知文に収録しています。元のLinkNestで公式SDKとのSHA-256一致を確認した同じ固定依存を利用しています。WebView2 Runtime本体は同梱せず、Microsoftの別の利用条件に従う共有Evergreen Runtimeを使用します。Runtimeがオープンソースであるという意味ではありません。

本体MITの根拠：[MIT原文](https://opensource.org/license/mit)。その他：[Apache原文](https://www.apache.org/licenses/LICENSE-2.0)、[Unicode原文](https://www.unicode.org/license.txt)、[SQLite公式](https://sqlite.org/copyright.html)。再配布物には本体LICENSEと第三者通知を保持してください。ソースの依存バージョンや配布コンポーネントを変更した場合は再確認が必要です。
