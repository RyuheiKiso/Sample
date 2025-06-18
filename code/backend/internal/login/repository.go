package login

import (
	"backend/internal/common"
	"context"
	"database/sql"
)

type userRepository struct {
	common.DBRepository[*User]
}

// --- DBRepositoryのCRUDをUser用にオーバーライド ---

// UserRepositoryは共通Repositoryインターフェースを拡張
type UserRepository interface {
	common.Repository
	FindByUsername(ctx context.Context, username string) (*User, error)
}

// --- DBRepositoryのCRUDをUser用にオーバーライド ---
func (r *userRepository) Create(ctx context.Context, entity interface{}) error {
	u, ok := entity.(*User)
	if !ok {
		return sql.ErrConnDone
	}
	_, err := r.DB.ExecContext(ctx, "INSERT INTO user (username, password) VALUES (?, ?)", u.Username, u.Password)
	return err
}

func (r *userRepository) Update(ctx context.Context, entity interface{}) error {
	u, ok := entity.(*User)
	if !ok {
		return sql.ErrConnDone
	}
	_, err := r.DB.ExecContext(ctx, "UPDATE user SET username = ?, password = ? WHERE id = ?", u.Username, u.Password, u.ID)
	return err
}

func (r *userRepository) Delete(ctx context.Context, id int) error {
	_, err := r.DB.ExecContext(ctx, "DELETE FROM user WHERE id = ?", id)
	return err
}

func NewUserRepository(db *sql.DB) UserRepository {
	return &userRepository{
		DBRepository: common.DBRepository[*User]{
			DB:    db,
			Table: "user",
			ScanRow: func(row *sql.Row) (*User, error) {
				var u User
				err := row.Scan(&u.ID, &u.Username, &u.Password)
				if err == sql.ErrNoRows {
					return nil, nil
				}
				if err != nil {
					return nil, err
				}
				return &u, nil
			},
		},
	}
}

func (r *userRepository) FindByUsername(ctx context.Context, username string) (*User, error) {
	row := r.DB.QueryRowContext(ctx, "SELECT id, username, password FROM user WHERE username = ?", username)
	return r.ScanRow(row)
}

// --- 共通Repositoryインターフェース実装はDBRepositoryに委譲 ---

type User struct {
	ID       int
	Username string
	Password string
}
