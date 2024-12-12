# react-axum-template

AxumでReactをサーブするテンプレートを作成するコマンド

```
cargo install react-axum-template
react-axum-template
```

## 特徴

- できるだけバージョン指定しないテンプレートのため、テンプレート自体のメンテナンスが直近で行われていなくても最新版のライブラリを使用できる
- 1つのAxumアプリで、サーバーサイドAPIとビルド済みのReactフロントエンドをルーティングする仕組みのため、1サーバーでアプリケーション全体が完結する
- 開発環境の2プロセスの立ち上げもTaskfileの1コマンドで可能
- E2Eテスト用のDockerComposeとPlaywrightが導入済み
- 本番ビルド用のDockerfileもあり

## スタック

### サーバーサイド

Axumベースでaxum-testやanyhow、tracing等が入っています。

### フロントエンド

FarmのReactテンプレートを用いてプロジェクトを生成します。パッケージマネージャはpnpmを使用します。biomeがセットアップ済みです。
