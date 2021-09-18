package data

import (
	"context"
	"go.mongodb.org/mongo-driver/bson/primitive"
	"os"

	"go.mongodb.org/mongo-driver/bson"
	"go.mongodb.org/mongo-driver/mongo"
	"log"
)

// HardConstraint : represents struct for a single HardConstraint
type HardConstraint struct {
	ID          primitive.ObjectID `json:"id" bson:"_id,omitempty"`
	DailyMax    int                `json:"daily_max"`
	WeeklyMax   int                `json:"weekly_max"`
	WeeklyMin   int                `json:"weekly_min"`
	MaxPerShift int                `json:"max_per_shift"`
}

func NewHardConstraint(dailyMax, weeklyMax, weeklyMin, maxPerShift int) *HardConstraint {
	hc := new(HardConstraint)
	hc.DailyMax = dailyMax
	hc.WeeklyMax = weeklyMax
	hc.WeeklyMin = weeklyMin
	hc.MaxPerShift = maxPerShift
	return hc
}

// SaveHardConstraint : save new hard constraint to MongoDB
func SaveHardConstraint(hc *HardConstraint) (*mongo.InsertOneResult, error) {

	insertResult, err := collectionHardConstraint.InsertOne(context.TODO(), hc)
	if err != nil {
		log.New(os.Stdout, "ERROR: ", log.Ltime|log.Lshortfile).Println(err)
		return insertResult, err
	}

	log.New(os.Stdout, "INFO: ", log.Ltime|log.Lshortfile).Println("HardConstraint insert success")
	return insertResult, nil

}

// GetHardConstraintById : return hard constraint by HardConstraint ID
func GetHardConstraintById(id primitive.ObjectID) (HardConstraint, error) {

	var hc HardConstraint

	err := collectionHardConstraint.FindOne(context.TODO(), bson.D{{"_id", id}}).Decode(&hc)

	if err != nil {
		log.New(os.Stdout, "ERROR: ", log.Ltime|log.Lshortfile).Println(err)
		return hc, err
	}

	log.New(os.Stdout, "INFO: ", log.Ltime|log.Lshortfile).Println("HardConstraint return success")
	return hc, nil
}

// UpdateHardConstraint : update hard constraint with new one
func UpdateHardConstraint(hc *HardConstraint) (*mongo.UpdateResult, error) {

	result, err := collectionHardConstraint.ReplaceOne(context.TODO(), bson.D{{"_id", hc.ID}}, hc)

	if err != nil {
		log.New(os.Stdout, "ERROR: ", log.Ltime|log.Lshortfile).Println(err)
		return result, err
	}

	log.New(os.Stdout, "INFO: ", log.Ltime|log.Lshortfile).Println("HardConstraint update success")
	return result, nil
}

// GetAllHardConstraints : return all hard constraints
func GetAllHardConstraints() ([]HardConstraint, error) {

	var hcs []HardConstraint

	cursor, err := collectionHardConstraint.Find(context.TODO(), bson.D{})
	if err != nil {
		log.New(os.Stdout, "ERROR: ", log.Ltime|log.Lshortfile).Println(err)
		return hcs, err
	}

	for cursor.Next(context.TODO()) {
		var hc HardConstraint
		err := cursor.Decode(&hc)
		if err != nil {
			log.New(os.Stdout, "ERROR: ", log.Ltime|log.Lshortfile).Println(err)
			return hcs, err
		}
		hcs = append(hcs, hc)
	}

	log.New(os.Stdout, "INFO: ", log.Ltime|log.Lshortfile).Println("HardConstraint return all success")
	return hcs, nil
}

// DeleteHardConstraint : delete hard constraint
func DeleteHardConstraint(id primitive.ObjectID) (*mongo.DeleteResult, error) {

	result, err := collectionHardConstraint.DeleteOne(context.TODO(), bson.D{{"_id", id}})
	if err != nil {
		log.New(os.Stdout, "ERROR: ", log.Ltime|log.Lshortfile).Println(err)
		return result, err
	}
	log.New(os.Stdout, "INFO: ", log.Ltime|log.Lshortfile).Println("HardConstraint delete success")
	return result, nil
}
