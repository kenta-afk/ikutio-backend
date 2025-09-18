# ベースイメージを指定
FROM amazon/dynamodb-local:latest

# rootユーザーに切り替えてAWS CLIをインストール
USER root

# yumとpipを使ってAWS CLIをインストール
RUN yum install -y python3-pip && \
    pip3 install awscli

# 処理完了後、元のユーザーに戻る（セキュリティ上の理由から推奨）
USER dynamodblocal