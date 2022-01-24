package data

import (
	"context"
	"github.com/romana98/NTP/enum"
	"github.com/romana98/NTP/logging"
	"go.mongodb.org/mongo-driver/bson"
	"go.mongodb.org/mongo-driver/bson/primitive"
	"go.mongodb.org/mongo-driver/mongo"
)

// SoftConstraint : represents struct for a single SoftConstraint
type SoftConstraint struct {
	ID      primitive.ObjectID `json:"id" bson:"_id,omitempty"`
	Prefers map[enum.DAY]int   `json:"prefers"`
}

func NewSoftConstraint(prefers map[enum.DAY]int) *SoftConstraint {
	sc := new(SoftConstraint)
	sc.Prefers = prefers
	return sc
}

// SaveSoftConstraint : save new soft constraint to MongoDB
func SaveSoftConstraint(sc *SoftConstraint) (*mongo.InsertOneResult, error) {

	insertResult, err := collectionSoftConstraint.InsertOne(context.TODO(), sc)
	if err != nil {
		logging.ErrorLogger.Println(err)
		return insertResult, err
	}

	logging.InfoLogger.Println("SoftConstraint insert success")
	return insertResult, nil
}

// GetSoftConstraintById : return soft constraint by SoftConstraint ID
func GetSoftConstraintById(id primitive.ObjectID) (SoftConstraint, error) {

	var sc SoftConstraint

	err := collectionSoftConstraint.FindOne(context.TODO(), bson.D{{"_id", id}}).Decode(&sc)

	if err != nil {
		logging.ErrorLogger.Println(err)
		return sc, err
	}

	logging.InfoLogger.Println("SoftConstraint insert success")
	return sc, nil
}

// UpdateSoftConstraint : update soft constraint with new one
func UpdateSoftConstraint(sc *SoftConstraint) (*mongo.UpdateResult, error) {

	result, err := collectionSoftConstraint.ReplaceOne(context.TODO(), bson.D{{"_id", sc.ID}}, sc)

	if err != nil {
		logging.ErrorLogger.Println(err)
		return result, err
	}

	logging.InfoLogger.Println("SoftConstraint insert success")
	return result, nil
}
