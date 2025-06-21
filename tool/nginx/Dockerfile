# nginx公式イメージを使用
FROM nginx:1.25-alpine


# 設定ファイル・証明書・鍵をコピー（ビルドコンテキスト直下から）
COPY tool/nginx/nginx.conf /etc/nginx/nginx.conf
COPY tool/nginx/cert.cer /etc/nginx/cert.cer
COPY tool/nginx/key.pem /etc/nginx/key.pem

# フロントエンドのビルド成果物をコピー（code/frontend/build）
COPY code/frontend/build /usr/share/nginx/html

EXPOSE 443
CMD ["nginx", "-g", "daemon off;"]
