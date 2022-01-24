package data

import (
	"context"
	"github.com/romana98/NTP/enum"
	"github.com/romana98/NTP/logging"
	"go.mongodb.org/mongo-driver/bson"
	"go.mongodb.org/mongo-driver/bson/primitive"
	"go.mongodb.org/mongo-driver/mongo"
)

// Shift : represents struct for a single Shift
type Shift struct {
	ID    primitive.ObjectID `json:"id" bson:"_id,omitempty"`
	Start string             `json:"start"`
	End   string             `json:"end"`
	Day   enum.DAY           `json:"day"`
}

func NewShift(start, end string, day enum.DAY) *Shift {
	s := new(Shift)
	s.Start = start
	s.End = end
	s.Day = day
	return s
}

// SaveShift : save new shift to MongoDB
func SaveShift(shift *Shift) (*mongo.InsertOneResult, error) {

	insertResult, err := collectionShift.InsertOne(context.TODO(), shift)
	if err != nil {
		logging.ErrorLogger.Println(err)
		return insertResult, err
	}

	logging.InfoLogger.Println("Shift insert success")
	return insertResult, nil
}

// GetShiftById : return shift by Shift ID
func GetShiftById(id primitive.ObjectID) (Shift, error) {

	var shift Shift

	err := collectionShift.FindOne(context.TODO(), bson.D{{"_id", id}}).Decode(&shift)

	if err != nil {
		logging.ErrorLogger.Println(err)
		return shift, err
	}

	logging.InfoLogger.Println("Shift insert success")
	return shift, nil
}

// UpdateShift : update shift with new one
func UpdateShift(shift *Shift) (*mongo.UpdateResult, error) {

	result, err := collectionShift.ReplaceOne(context.TODO(), bson.D{{"_id", shift.ID}}, shift)

	if err != nil {
		logging.ErrorLogger.Println(err)
		return result, err
	}

	logging.InfoLogger.Println("Shift insert success")
	return result, nil
}

// GetAllShifts : return all shifts
func GetAllShifts() ([]Shift, error) {

	var shift Shift
	var shifts []Shift

	cursor, err := collectionShift.Find(context.TODO(), bson.D{})
	if err != nil {
		logging.ErrorLogger.Println(err)
		return shifts, err
	}

	for cursor.Next(context.TODO()) {
		err := cursor.Decode(&shift)
		if err != nil {

			logging.ErrorLogger.Println(err)
			return shifts, err
		}
		shifts = append(shifts, shift)
	}

	logging.InfoLogger.Println("Shift return all success")
	return shifts, nil
}

// GetAllShiftsByFaculty : return all by faculty
func GetAllShiftsByFaculty(ids []primitive.ObjectID) ([]Shift, error) {

	var shifts []Shift

	cursor, err := collectionShift.Find(context.TODO(), bson.M{"_id": bson.M{"$in": ids}})
	if err != nil {
		logging.ErrorLogger.Println(err)
		return shifts, err
	}

	for cursor.Next(context.TODO()) {
		var shift Shift
		err := cursor.Decode(&shift)
		if err != nil {

			logging.ErrorLogger.Println(err)
			return shifts, err
		}
		shifts = append(shifts, shift)
	}

	logging.InfoLogger.Println("Shift return all by faculty success")
	return shifts, nil
}

// DeleteShift : delete shift
func DeleteShift(id primitive.ObjectID) (*mongo.DeleteResult, error) {

	result, err := collectionShift.DeleteOne(context.TODO(), bson.D{{"_id", id}})
	if err != nil {
		logging.ErrorLogger.Println(err)
		return result, err
	}

	logging.InfoLogger.Println("Shift delete success")
	return result, nil
}
