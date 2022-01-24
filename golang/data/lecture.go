package data

import (
	"context"
	"github.com/romana98/NTP/logging"
	"go.mongodb.org/mongo-driver/bson"
	"go.mongodb.org/mongo-driver/bson/primitive"
	"go.mongodb.org/mongo-driver/mongo"
)

// Lecture : represents struct for a single Lecture
type Lecture struct {
	ID            primitive.ObjectID `json:"id" bson:"_id,omitempty"`
	Name          string             `json:"name"`
	NumberOfTimes int                `json:"number_of_times"`
}

func NewLecture(numberOfTimes int, name string) *Lecture {
	lecture := new(Lecture)
	lecture.Name = name
	lecture.NumberOfTimes = numberOfTimes
	return lecture
}

// SaveLecture : save new lecture to MongoDB
func SaveLecture(lecture *Lecture) (*mongo.InsertOneResult, error) {

	insertResult, err := collectionLecture.InsertOne(context.TODO(), lecture)
	if err != nil {
		logging.ErrorLogger.Println(err)
		return insertResult, err
	}

	logging.InfoLogger.Println("Lecture update success")
	return insertResult, nil
}

// GetLectureById : return lecture by Lecture ID
func GetLectureById(id primitive.ObjectID) (Lecture, error) {

	var lecture Lecture

	err := collectionLecture.FindOne(context.TODO(), bson.D{{"_id", id}}).Decode(&lecture)

	if err != nil {
		logging.ErrorLogger.Println(err)
		return lecture, err
	}

	logging.InfoLogger.Println("Lecture update success")
	return lecture, nil
}

// UpdateLecture : update lecture with new one
func UpdateLecture(lecture *Lecture) (*mongo.UpdateResult, error) {

	result, err := collectionLecture.ReplaceOne(context.TODO(), bson.D{{"_id", lecture.ID}}, lecture)

	if err != nil {
		logging.ErrorLogger.Println(err)
		return result, err
	}

	logging.InfoLogger.Println("Lecture update success")
	return result, nil
}

// GetAllLectures : return all lectures
func GetAllLectures() ([]Lecture, error) {

	var lectures []Lecture

	cursor, err := collectionLecture.Find(context.TODO(), bson.D{})
	if err != nil {
		logging.ErrorLogger.Println(err)
		return lectures, err
	}

	for cursor.Next(context.TODO()) {
		var lecture Lecture
		err := cursor.Decode(&lecture)
		if err != nil {

			logging.ErrorLogger.Println(err)
			return lectures, err
		}
		lectures = append(lectures, lecture)
	}

	logging.InfoLogger.Println("Lecture return all success")
	return lectures, nil
}

// GetAllLecturesByIds : return lectures by professor ID
func GetAllLecturesByIds(ids []primitive.ObjectID) ([]Lecture, error) {

	var lectures []Lecture

	cursor, err := collectionLecture.Find(context.TODO(), bson.M{"_id": bson.M{"$in": ids}})
	if err != nil {
		logging.ErrorLogger.Println(err)
		return lectures, err
	}

	for cursor.Next(context.TODO()) {
		var lecture Lecture
		err := cursor.Decode(&lecture)
		if err != nil {

			logging.ErrorLogger.Println(err)
			return lectures, err
		}
		lectures = append(lectures, lecture)
	}

	logging.InfoLogger.Println("Lecture return all by professor success")
	return lectures, nil
}

// DeleteLecture : delete lecture
func DeleteLecture(id primitive.ObjectID) (*mongo.DeleteResult, error) {

	result, err := collectionLecture.DeleteOne(context.TODO(), bson.D{{"_id", id}})
	if err != nil {
		logging.ErrorLogger.Println(err)
		return result, err
	}

	logging.InfoLogger.Println("Lecture delete success")
	return result, nil
}
