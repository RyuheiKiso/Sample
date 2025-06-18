package login

import (
	"database/sql"
)

type UserRepository interface {
	FindByUsername(username string) (*User, error)
}

type userRepository struct {
	db *sql.DB
}

func NewUserRepository(db *sql.DB) UserRepository {
	return &userRepository{db: db}
}

func (r *userRepository) FindByUsername(username string) (*User, error) {
	row := r.db.QueryRow("SELECT id, username, password FROM user WHERE username = ?", username)
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

type User struct {
	ID       int
	Username string
	Password string
}
