package common

import "context"

// Service は全サービス共通のインターフェース例
// 必要に応じて拡張可能
// 例: HealthCheck, 共通メソッドなど

type Service interface {
	HealthCheck(ctx context.Context) error
}

// BaseService は共通フィールドを持つベース構造体例
// 今後必要に応じてフィールド追加可

type BaseService struct {
	// 共通フィールド例: ロガー, 設定, etc.
}

// HealthCheck のデフォルト実装例
func (b *BaseService) HealthCheck(ctx context.Context) error {
	// 必要に応じてDB疎通や外部サービス疎通など
	return nil
}
