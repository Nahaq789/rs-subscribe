#!/bin/bash
set -e

# .envファイルが存在する場合に読み込む
if [ -f .env ]; then
    source .env
else
    echo "Error: .env file not found"
    echo "Please copy .env.example to .env and set appropriate values"
    exit 1
fi

# 必要な環境変数のチェック
required_envs=("AWS_REGION" "AWS_PROFILE" "AWS_ACCOUNT_ID" "REPOSITORY_NAME")
for env in "${required_envs[@]}"; do
    if [ -z "${!env}" ]; then
        echo "Error: $env is not set in .env file"
        exit 1
    fi
done

REGION="${AWS_REGION}"
PROFILE="${AWS_PROFILE}"
ACCOUNT_ID="${AWS_ACCOUNT_ID}"
REPOSITORY_NAME="${REPOSITORY_NAME}"
IMAGE_TAG="${IMAGE_TAG:-latest}"

echo "Deploying to ECR..."
echo "Region: $REGION"
echo "Repository: $REPOSITORY_NAME"
echo "Tag: $IMAGE_TAG"

# ECRログイン
aws ecr get-login-password \
    --region $REGION \
    --profile $PROFILE \
    | docker login \
    --username AWS \
    --password-stdin $ACCOUNT_ID.dkr.ecr.$REGION.amazonaws.com

# イメージビルド
docker build --platform=linux/amd64 -t $REPOSITORY_NAME:$IMAGE_TAG .

# タグ付け
docker tag $REPOSITORY_NAME:$IMAGE_TAG \
    $ACCOUNT_ID.dkr.ecr.$REGION.amazonaws.com/$REPOSITORY_NAME:$IMAGE_TAG

# プッシュ
docker push $ACCOUNT_ID.dkr.ecr.$REGION.amazonaws.com/$REPOSITORY_NAME:$IMAGE_TAG

echo "Deploy completed successfully!"