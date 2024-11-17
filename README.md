# Saddy
* * *

<a alt="rust: v1.80.1" href="https://www.rust-lang.org">
 <img src="https://img.shields.io/badge/rust-v1.80.1-orange.svg">
</a>
<a alt="terraform: v1.9.6" href="https://www.terraform.io">
 <img src="https://img.shields.io/badge/terraform-v1.9.6-7B42BC.svg">
</a>
<a alt="docker" href="https://www.docker.com">
 <img src="https://img.shields.io/badge/docker-2496ED.svg">
</a>
<a alt="AWS Lambda" href="https://aws.amazon.com/lambda/">
  <img src="https://img.shields.io/badge/Lambda-FF9900.svg">
</a>
<a alt="Amazon DynamoDB" href="https://aws.amazon.com/dynamodb/">
  <img src="https://img.shields.io/badge/DynamoDB-4053D6.svg">
</a>
<a alt="Amazon ECR" href="https://aws.amazon.com/ecr/">
  <img src="https://img.shields.io/badge/ECR-FF9900.svg">
</a>
<a alt="GitHub Actions" href="https://github.com/features/actions">
 <img src="https://img.shields.io/badge/github_actions-2088FF.svg">
</a>
<a alt="Build Status" href="https://github.com/Nahaq789/rs-subscribe/actions">
 <img src="https://github.com/Nahaq789/rs-subscribe/workflows/rs-subscribe CI/badge.svg">
</a>
<a alt="MIT License" href="https://opensource.org/licenses/MIT">
 <img src="https://img.shields.io/badge/license-MIT-blue.svg">
</a>

## 環境設定

1. 環境変数ファイルの準備
```bash
AWS_REGION=*
AWS_PROFILE=dev
AWS_ACCOUNT_ID=*
REPOSITORY_NAME=*
IMAGE_TAG=dev or latest
```

## 認証情報の管理
- ローカル開発: AWS CLI credentials (`~/.aws/credentials`) を使用
- Lambda実行時: IAMロールによる権限管理
- terraform.tfvars にはインフラ構成の設定のみを含み、機密情報は含まない