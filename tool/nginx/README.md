# nginx（リバースプロキシ）Dockerイメージ ビルド・起動手順

このドキュメントは、nginxを使ったリバースプロキシ用Dockerイメージのビルド・起動手順をまとめたものです。

---

## 前提条件
- Dockerがインストールされていること
- このリポジトリのルートディレクトリで作業すること
- Windows環境の場合、ホストPCのローカルIPアドレス（例: 192.168.1.9）がnginx.confに正しく設定されていること

---

## ビルド手順

1. コマンドプロンプトまたはPowerShellで、リポジトリのルートディレクトリに移動します。

2. 以下のコマンドでnginxイメージをビルドします。

```
docker build --pull --rm -f tool\nginx\Dockerfile -t sample_nginx:latest .
```

- `--pull` : ベースイメージを常に最新に更新
- `--rm`   : 中間コンテナを自動削除
- `-f`     : Dockerfileのパス指定
- `-t`     : イメージ名:タグ
- `.`      : ビルドコンテキスト（リポジトリルート）

---

## 起動手順

1. 以下のコマンドでnginxコンテナを起動します。

```
docker run --rm -d -p 443:443 --name sample_nginx sample_nginx:latest
```

- `-p 443:443` : ホストの443番ポートをコンテナの443番に割り当て
- `--name`     : コンテナ名
- `-d`         : バックグラウンド実行
- `--rm`       : 停止時に自動削除

2. ブラウザで `https://[ホストPCのIPアドレス]/` にアクセスしてください。

---

## 注意事項
- Windowsファイアウォールで50051番ポート（バックエンド用）の受信許可が必要です。
- nginx.confの`proxy_pass`はホストPCの正しいIPアドレスに設定してください。
- 証明書エラーが出る場合は自己署名証明書のためです。開発用途でご利用ください。

---

## コンテナ停止

```
docker stop sample_nginx
```

---

## 参考
- `tool/nginx/nginx.conf` ... nginxの設定ファイル
- `tool/nginx/cert.cer`, `tool/nginx/key.pem` ... SSL証明書・秘密鍵
- `code/frontend/build` ... フロントエンドのビルド成果物
