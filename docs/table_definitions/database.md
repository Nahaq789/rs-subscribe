# データベース設計（DynamoDB）

## ユーザーテーブル
```json
{
  "userId": "usr_550e8400-e29b-41d4-a716-446655440000",  // Partition Key
  "countryId": 1,        // Integer
  "createdAt": "2024-10-24T10:00:00Z",  // ISO 8601
  "updatedAt": "2024-10-24T10:00:00Z"   // ISO 8601
}
```

## 国マスタテーブル
```json
{
  "countryId": 1,         // Partition Key (Integer)
  "countryCode": "JP",    // ISO 3166-1 alpha-2
  "countryName": "日本",
  "currencyCode": "JPY",  // ISO 4217
  "currencySymbol": "¥",
  "localeCode": "ja_JP"
}
```

## サブスクテーブル
```json
{
  "userId": "usr_550e8400-e29b-41d4-a716-446655440000",      // Partition Key
  "subscriptionId": "sub_123e4567-e89b-12d3-a456-426614174000", // Sort Key
  "name": "Netflix",
  "paymentMethodId": "pay_987f6543-e89b-12d3-a456-426614174000",
  "amount": 1980,          // Integer
  "paymentCycle": "MONTHLY", // MONTHLY, YEARLY, etc
  "categoryId": "cat_abc12345-e89b-12d3-a456-426614174000",
  "iconLocalPath": "subscription_sub123",
  "notification": true,     // Boolean
  "firstPaymentDate": "2024-01-01T00:00:00Z",
  "nextPaymentDate": "2024-11-01T00:00:00Z",
  "autoRenewal": true,     // Boolean
  "status": 1,             // 1: 利用中, 2: 一時停止, 3: 解約
  "memo": "家族プラン",
  "createdAt": "2024-10-24T10:00:00Z",
  "updatedAt": "2024-10-24T10:00:00Z"
}
```

## カテゴリテーブル
```json
{
  "categoryId": "cat_abc12345-e89b-12d3-a456-426614174000", // Partition Key
  "sortOrder": 1,        // Integer
  "name": "動画配信",
  "createdAt": "2024-10-24T10:00:00Z",
  "updatedAt": "2024-10-24T10:00:00Z"
}
```

## 支払方法テーブル
```json
{
  "paymentMethodId": "pay_987f6543-e89b-12d3-a456-426614174000", // Partition Key
  "methodName": "CREDIT_CARD",
  "detailName": "Visa",
  "additionalName": "楽天カード",
  "createdAt": "2024-10-24T10:00:00Z",
  "updatedAt": "2024-10-24T10:00:00Z"
}
```