package controllers

import (
	"context"
	"fmt"

	"github.com/fnhernandorena/golang210324/database"
	"github.com/fnhernandorena/golang210324/models"
	"github.com/gofiber/fiber/v2"
	"go.mongodb.org/mongo-driver/bson"
	"go.mongodb.org/mongo-driver/bson/primitive"
)

func GetShoes(c *fiber.Ctx) error {
	var shoes []models.Shoe
	results, err := database.Collection.Find(context.TODO(), bson.M{})
	if err != nil {
		return err
	}
	defer results.Close(context.TODO())

	for results.Next(context.TODO()) {
		var shoe models.Shoe
		err := results.Decode(&shoe)
		if err != nil {
			return err
		}
		shoes = append(shoes, shoe)
	}

	return c.JSON(&fiber.Map{
		"shoes": shoes,
	})
}

func CreateShoe(c *fiber.Ctx) error {
	var shoe models.Shoe
	c.BodyParser(&shoe)
	document := bson.D{
		{Key: "brand", Value: shoe.Brand},
		{Key: "model", Value: shoe.Model},
		{Key: "size", Value: shoe.Size},
	}
	result, err := database.Collection.InsertOne(context.TODO(), document)

	if err != nil {
		fmt.Println(err)
	}

	return c.JSON(&fiber.Map{
		"data": result,
	})
}
func GetShoe(c *fiber.Ctx) error {
	shoeID := c.Params("id")

	objID, err := primitive.ObjectIDFromHex(shoeID)
	if err != nil {
		return c.Status(fiber.StatusBadRequest).JSON(fiber.Map{
			"error": "Shoe ID is not valid",
		})
	}

	filter := bson.M{"_id": objID}
	var shoe models.Shoe
	err = database.Collection.FindOne(context.TODO(), filter).Decode(&shoe)
	if err != nil {
		return c.Status(fiber.StatusInternalServerError).JSON(fiber.Map{
			"error": "Error retrieving shoe",
		})
	}

	return c.JSON(shoe)
}

func DeleteShoe(c *fiber.Ctx) error {
	userID := c.Params("id")

	objID, err := primitive.ObjectIDFromHex(userID)
	if err != nil {
		return c.Status(fiber.StatusBadRequest).JSON(fiber.Map{
			"error": "Shoe id not valid",
		})
	}

	filter := bson.M{"_id": objID}

	_, err = database.Collection.DeleteOne(context.TODO(), filter)
	if err != nil {
		return c.Status(fiber.StatusInternalServerError).JSON(fiber.Map{
			"error": "Error at delete shoe",
		})
	}

	return c.JSON(fiber.Map{
		"message": "Shoe deleted successfully",
	})
}
func UpdateShoe(c *fiber.Ctx) error {
	shoeID := c.Params("id")
	objID, err := primitive.ObjectIDFromHex(shoeID)
	if err != nil {
		return c.Status(fiber.StatusBadRequest).JSON(fiber.Map{
			"error": "Shoe ID is not a valid",
		})
	}
	var updatedShoe models.Shoe
	if err := c.BodyParser(&updatedShoe); err != nil {
		return c.Status(fiber.StatusBadRequest).JSON(fiber.Map{
			"error": "Error at request shoe data",
		})
	}
	filter := bson.M{"_id": objID}
	update := bson.D{
		{Key: "brand", Value: updatedShoe.Brand},
		{Key: "model", Value: updatedShoe.Model},
		{Key: "size", Value: updatedShoe.Size},
	}
	ee, err := database.Collection.UpdateOne(context.TODO(), filter, update)
	if err != nil {
		return c.Status(fiber.StatusInternalServerError).JSON(fiber.Map{
			"error": "Error at update shoe",
		})
	}
	fmt.Printf("UpdateResult: %+v\n", ee)

	return c.JSON(fiber.Map{
		"message": "Shoe updated successfully",
	})
}
