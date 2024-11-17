provider "aws" {
  region = var.aws_region
  profile = "dev"
}

module "dynamodb" {
  source = "../../modules/dynamodb"

  environment = var.environment
  tables     = var.dynamodb_tables
}

module "ecr" {
  source = "../../modules/ecr"

  environment      = var.environment
  repository_name  = "rs-subscribe-saddy-repository"
}

module "lambda" {
  source = "../../modules/lambda"

  environment     = var.environment
  function_name   = "rs-subscribe-saddy"
  lambda_role_arn = aws_iam_role.lambda_iam_role.arn
  ecr_repository_url = module.ecr.repository_url

  environment_variables = {
    PAYMENT_TABLE = module.dynamodb.table_names["payment"]
    RUST_BACKTRACE = "1"
    RUST_LOG       = "info"
    HOST           = "0.0.0.0"
    PORT           = "8080"
  }
}

# Lambda用のIAMロール
resource "aws_iam_role" "lambda_iam_role" {
  name = "${var.environment}_subscribe_lambda_iam_role"

  assume_role_policy = jsonencode({
    Version = "2012-10-17"
    Statement = [
      {
        Action = "sts:AssumeRole"
        Principal = {
          Service = "lambda.amazonaws.com"
        }
        Effect = "Allow"
        Sid    = ""
      }
    ]
  })

  tags = {
    Environment = var.environment
  }
}

# Lambda用Policyの作成
resource "aws_iam_role_policy" "lambda_access_policy" {
  name = "${var.environment}_lambda_access_policy"
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
      },
      {
        Effect = "Allow"
        Action = [
          "dynamodb:PutItem",
          "dynamodb:GetItem",
          "dynamodb:UpdateItem",
          "dynamodb:DeleteItem",
          "dynamodb:Query",
          "dynamodb:Scan"
        ]
        Resource = values(module.dynamodb.table_arns)  # すべてのテーブルのARNを取得
      }
    ]
  })
}

# Lambda基本実行ロールのポリシーをアタッチ
resource "aws_iam_role_policy_attachment" "lambda_basic_execution" {
  policy_arn = "arn:aws:iam::aws:policy/service-role/AWSLambdaBasicExecutionRole"
  role       = aws_iam_role.lambda_iam_role.name
}