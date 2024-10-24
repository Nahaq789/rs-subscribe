# データベース設計（DynamoDB）

## ユーザーテーブル
```json
{
  "userId": "user123",    // Partition Key（CognitoのユーザーID）
  "countryId": 1,        // Integer
  "createdAt": "2024-10-24",
  "updatedAt": "2024-10-24"
}
```

## 国マスタテーブル
```json
{
  "countryId": 1,         // Partition Key (Integer)
  "countryCode": "JP",    // 国コード（ISO 3166-1 alpha-2）
  "countryName": "日本",
  "currencyCode": "JPY",  // ISO 4217
  "currencySymbol": "¥",
  "localeCode": "ja_JP"
}
```

## サブスクテーブル
```json
{
  "subscriptionId": "sub123", // Partition Key
  "userId": "user123",      // ユーザーID
  "name": "Netflix",        // サブスク名
  "paymentMethodId": "pay123", // 支払方法ID
  "amount": 1980,          // 金額
  "paymentCycle": "MONTHLY", // 支払周期
  "categoryId": "cat123",   // カテゴリID
  "iconLocalPath": "subscription_sub123", // アイコンのローカルパス
  "notification": true,     // 通知設定
  "firstPaymentDate": "2024-01-01", // 初回支払日
  "nextPaymentDate": "2024-11-01", // 次回支払予定日
  "autoRenewal": true,     // 自動更新フラグ
  "status": 1,      // ステータス（1: ACTIVE/2: PAUSED/3: CANCELLED）
  "memo": "家族プラン"      // メモ欄
}
```

## カテゴリテーブル
```json
{
  "categoryId": "cat123", // Partition Key
  "sortOrder": 1,        // ソート番号
  "name": "動画配信"      // カテゴリ名
}
```

## 支払方法テーブル
```json
{
  "paymentMethodId": "pay123", // Partition Key
  "paymentDetailId": "detail123", // 支払方法詳細ID
  "name": "クレジットカード"    // 支払方法名
}
```

## 支払方法詳細テーブル
```json
{
  "paymentDetailId": "detail123", // Partition Key
  "name": "Visa",               // カード会社名等
  "additionalName": "楽天カード" // 追加名称（Null許容）
}
```