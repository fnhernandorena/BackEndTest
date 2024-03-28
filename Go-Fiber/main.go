package main

import (
	"log"

	"github.com/fnhernandorena/golang210324/controllers"
	"github.com/fnhernandorena/golang210324/database"
	"github.com/gofiber/fiber/v2"
)

func main() {
	if err := database.InitDatabase(); err != nil {
		log.Fatalf("Error inicializando base de datos: %v", err)
	}

	app := fiber.New()

	app.Get("/shoes", controllers.GetShoes)
	app.Post("/shoes", controllers.CreateShoe)
	app.Get("/shoes/:id", controllers.GetShoe)
	app.Delete("/shoes/:id", controllers.DeleteShoe)
	app.Put("/shoes/:id", controllers.UpdateShoe)

	log.Fatal(app.Listen(":3001"))
}
