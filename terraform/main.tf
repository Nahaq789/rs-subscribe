module "ecr" {
  source = "./ecr"
}

module "lambda" {
  source = "./lambda"
  lambda_role_arn = aws_iam_role.lambda_iam_role.arn
  ecr_repository_url = module.ecr.repository_url
}

# AWSプロバイダの設定
provider "aws" {
  region     = var.region
  access_key = var.access_key
  secret_key = var.secret_key
}

# lambda用のRole設定
resource "aws_iam_role" "lambda_iam_role" {
  name               = "subscribe_lambda_iam_role"
  assume_role_policy = <<POLICY
  {
    "Version": "2012-10-17",
    "Statement": [
      {
        "Action": "sts:AssumeRole",
        "Principal": {
          "Service": "lambda.amazonaws.com"
        },
        "Effect": "Allow",
        "Sid": ""
      }
    ]
  }
  POLICY
}

# Lambda用Policyの作成
resource "aws_iam_role_policy" "lambda_access_policy" {
  name = "terraform_lambda_access_policy"
  role = aws_iam_role.lambda_iam_role.id
  policy = jsonencode({
    Version = "2012-10-17"
    Statement = [
      {
        Effect = "Allow"
        Action = [
          "logs:CreateLogStream",
          "logs:CreateLogGroup",
          "logs:PutLogEvents"
        ]
        Resource = "arn:aws:logs:*:*:*"
      },
      {
        Effect = "Allow"
        Action = [
          "ecr:GetAuthorizationToken"
        ]
        Resource = "*"
      },
      {
        Effect = "Allow"
        Action = [
          "ecr:BatchCheckLayerAvailability",
          "ecr:GetDownloadUrlForLayer",
          "ecr:BatchGetImage"
        ]
        Resource = [
          module.ecr.repository_arn
        ]
      }
    ]
  })
}

# Lambda基本実行ロールのポリシーをアタッチ（オプション）
resource "aws_iam_role_policy_attachment" "lambda_basic_execution" {
  policy_arn = "arn:aws:iam::aws:policy/service-role/AWSLambdaBasicExecutionRole"
  role       = aws_iam_role.lambda_iam_role.name
}