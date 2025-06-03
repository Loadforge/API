package main

import (
	"time"

	"github.com/gin-contrib/cors"
	"github.com/gin-gonic/gin"
)

func main() {

	server := gin.Default()
	server.Use(cors.New(cors.Config{
		AllowOrigins:     []string{"*"}, // ou "*" para todas as origens
		AllowMethods:     []string{"GET", "POST", "PUT", "DELETE"},
		AllowHeaders:     []string{"Origin", "Content-Type", "Authorization"},
		ExposeHeaders:    []string{"Content-Length"},
		AllowCredentials: true,
		MaxAge:           12 * time.Hour,
	}))
	server.GET("/health", func(ctx *gin.Context) {
		ctx.JSON(200, gin.H{
			"message": "up",
		})
	})

	server.Run(":8080")
}
