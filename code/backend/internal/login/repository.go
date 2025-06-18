package login

import (
	"backend/internal/common"
	"context"
	"database/sql"
)

// UserRepositoryは共通Repositoryインターフェースを拡張
type UserRepository interface {
	common.Repository
	FindByUsername(ctx context.Context, username string) (*User, error)
}

type userRepository struct {
	db *sql.DB
}

func NewUserRepository(db *sql.DB) UserRepository {
	return &userRepository{db: db}
}

func (r *userRepository) FindByUsername(ctx context.Context, username string) (*User, error) {
	row := r.db.QueryRowContext(ctx, "SELECT id, username, password FROM user WHERE username = ?", username)
	var u User
	err := row.Scan(&u.ID, &u.Username, &u.Password)
	if err == sql.ErrNoRows {
		return nil, nil
	}
	if err != nil {
		return nil, err
	}
	return &u, nil
}

// --- 共通Repositoryインターフェース実装 ---
func (r *userRepository) GetByID(ctx context.Context, id int) (interface{}, error) {
	row := r.db.QueryRowContext(ctx, "SELECT id, username, password FROM user WHERE id = ?", id)
	var u User
	err := row.Scan(&u.ID, &u.Username, &u.Password)
	if err == sql.ErrNoRows {
		return nil, nil
	}
	if err != nil {
		return nil, err
	}
	return &u, nil
}

func (r *userRepository) Create(ctx context.Context, entity interface{}) error {
	u, ok := entity.(*User)
	if !ok {
		return sql.ErrConnDone // 型エラーの例
	}
	_, err := r.db.ExecContext(ctx, "INSERT INTO user (username, password) VALUES (?, ?)", u.Username, u.Password)
	return err
}

func (r *userRepository) Update(ctx context.Context, entity interface{}) error {
	u, ok := entity.(*User)
	if !ok {
		return sql.ErrConnDone
	}
	_, err := r.db.ExecContext(ctx, "UPDATE user SET username = ?, password = ? WHERE id = ?", u.Username, u.Password, u.ID)
	return err
}

func (r *userRepository) Delete(ctx context.Context, id int) error {
	_, err := r.db.ExecContext(ctx, "DELETE FROM user WHERE id = ?", id)
	return err
}

type User struct {
	ID       int
	Username string
	Password string
}
