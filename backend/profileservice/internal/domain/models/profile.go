package models

import (
	"time"
)

type Profile struct {
	ID UserId
	Name string
	CreatedAt time.Time
	UpdatedAt time.Time
}

func New(id UserId, name string) *Profile {
	return &Profile{
		ID: id,
		Name: name,
	}
}

func (p *Profile) UpdateName(name string) {
    p.Name = name  
    p.UpdatedAt = time.Now()  
}  