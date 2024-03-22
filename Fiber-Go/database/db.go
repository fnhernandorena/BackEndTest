package database

import (
	"context"
	"fmt"
	"os"

	"github.com/joho/godotenv"
	"go.mongodb.org/mongo-driver/mongo"
	"go.mongodb.org/mongo-driver/mongo/options"
)

var Collection *mongo.Collection

func InitDatabase() error {
	if err := godotenv.Load(); err != nil {
		return fmt.Errorf("error cargando archivo .env: %v", err)
	}
	mongodb := os.Getenv("MONGODB_URI")

	client, err := mongo.Connect(context.TODO(), options.Client().ApplyURI(mongodb))
	if err != nil {
		return fmt.Errorf("error conectando a MongoDB: %v", err)
	}

	Collection = client.Database("test").Collection("shoes")
	return nil
}
