package data

import (
	"context"
	"github.com/romana98/NTP/logging"
	"go.mongodb.org/mongo-driver/bson"
	"go.mongodb.org/mongo-driver/bson/primitive"
	"go.mongodb.org/mongo-driver/mongo"
)

// Faculty : represents struct for a single Faculty
type Faculty struct {
	ID             primitive.ObjectID   `json:"id" bson:"_id,omitempty"`
	Name           string               `json:"name"`
	HardConstraint primitive.ObjectID   `json:"hard_constraint"`
	Shifts         []primitive.ObjectID `json:"shifts"`
	Staff          []primitive.ObjectID `json:"staff"`
	Lectures       []primitive.ObjectID `json:"lectures"`
	Schedule       primitive.ObjectID   `json:"schedule"`
}

func NewFaculty(name string, hc, schedule primitive.ObjectID, shifts, staff, lectures []primitive.ObjectID) *Faculty {
	faculty := new(Faculty)
	faculty.Name = name
	faculty.HardConstraint = hc
	faculty.Shifts = shifts
	faculty.Staff = staff
	faculty.Lectures = lectures
	faculty.Schedule = schedule

	return faculty
}

// SaveFaculty : save new faculty to MongoDB
func SaveFaculty(Faculty *Faculty) (*mongo.InsertOneResult, error) {

	insertResult, err := collectionFaculty.InsertOne(context.TODO(), Faculty)
	if err != nil {
		logging.ErrorLogger.Println(err)
		return insertResult, err
	}

	logging.ErrorLogger.Println("Faculty insert success")
	return insertResult, nil
}

// GetFacultyById : return faculty by faculty ID
func GetFacultyById(id primitive.ObjectID) (Faculty, error) {

	var faculty Faculty

	err := collectionFaculty.FindOne(context.TODO(), bson.D{{"_id", id}}).Decode(&faculty)

	if err != nil {
		logging.ErrorLogger.Println(err)
		return faculty, err
	}

	logging.ErrorLogger.Println("Faculty return success")
	return faculty, nil
}

// UpdateFaculty : update faculty with new one
func UpdateFaculty(faculty *Faculty) (*mongo.UpdateResult, error) {

	result, err := collectionFaculty.ReplaceOne(context.TODO(), bson.D{{"_id", faculty.ID}}, faculty)

	if err != nil {
		logging.ErrorLogger.Println(err)
		return result, err
	}

	logging.InfoLogger.Println("Faculty update success")
	return result, nil
}

// GetAllFaculties : return all Faculties
func GetAllFaculties() ([]Faculty, error) {

	var faculties []Faculty

	cursor, err := collectionFaculty.Find(context.TODO(), bson.D{})
	if err != nil {
		logging.ErrorLogger.Println(err)
		return faculties, err
	}

	for cursor.Next(context.TODO()) {
		var faculty Faculty
		err := cursor.Decode(&faculty)
		if err != nil {

			logging.ErrorLogger.Println(err)
			return faculties, err
		}
		faculties = append(faculties, faculty)
	}

	logging.InfoLogger.Println("Faculty return all success")
	return faculties, nil
}

// DeleteFaculty : delete faculty
func DeleteFaculty(id primitive.ObjectID) (*mongo.DeleteResult, error) {

	result, err := collectionFaculty.DeleteOne(context.TODO(), bson.D{{"_id", id}})
	if err != nil {
		logging.ErrorLogger.Println(err)
		return result, err
	}

	logging.InfoLogger.Println("Faculty delete success")
	return result, nil
}
