package models

import (
	"github.com/gofiber/fiber/v2"
	"github.com/golang-jwt/jwt/v5"
)

type RoleLevel = int8

const (
	OWNER RoleLevel = iota
	ADMIN
	USER
)

type User struct {
	Id        string    `json:"id"`
	Email     string    `json:"email"`
	Username  string    `json:"username"`
	Token     string    `json:"token"`
	Role      RoleLevel `json:"role"`
	CreatedAt string    `json:"created_at"`
}

type Author struct {
	Email string `json:"email"`
	Name  string `json:"name"`
	Url   string `json:"url"`
}

type AppInfo struct {
	Name     string        `json:"name"`
	Version  string        `json:"version"`
	Homepage string        `json:"homepage"`
	Bugs     string        `json:"bugs"`
	Author   Author        `json:"author"`
	Routes   []fiber.Route `json:"routes"`
}

type ErrorResponse struct {
	Success bool   `json:"success"`
	Code    int    `json:"code"`
	Message string `json:"message"`
}

type JWTClaims struct {
	Id string `json:"id"`
	jwt.RegisteredClaims
}

type Config struct {
	Host      string  `json:"host"`
	JWTSecret string  `json:"jwt_secret"`
	App       AppInfo `json:"app"`
}
