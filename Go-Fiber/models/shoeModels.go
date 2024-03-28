package models

import "go.mongodb.org/mongo-driver/bson/primitive"

type Shoe struct {
	Id    primitive.ObjectID `json:"_id" bson:"_id"`
	Brand string             `json:"Brand"`
	Model string             `json:"Model"`
	Size  int                `json:"Size"`
}
