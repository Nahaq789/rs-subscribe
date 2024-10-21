FROM rust:1.80.1-slim-bullseye as builder
WORKDIR /usr/src/app
COPY . .
# 依存関係のビルド（キャッシュのため）
RUN cargo build --release
# 実際のアプリケーションのビルド
RUN cargo build --release

# 実行ステージ
FROM gcr.io/distroless/cc-debian12
# ビルドしたバイナリをコピー
COPY --from=builder /usr/src/app/target/release/server /var/runtime/bootstrap
# Lambda web adapterをコピー
COPY --from=public.ecr.aws/awsguru/aws-lambda-adapter:0.8.4 /lambda-adapter /opt/extensions/lambda-adapter
CMD ["/var/runtime/bootstrap"]