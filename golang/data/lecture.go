package data

import (
	"context"
	"go.mongodb.org/mongo-driver/bson/primitive"
	"os"

	"go.mongodb.org/mongo-driver/bson"
	"go.mongodb.org/mongo-driver/mongo"
	"log"
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
		log.New(os.Stdout, "ERROR: ", log.Ltime|log.Lshortfile).Println(err)
		return insertResult, err
	}

	log.New(os.Stdout, "INFO: ", log.Ltime|log.Lshortfile).Println("Lecture insert success")
	return insertResult, nil

}

// GetLectureById : return lecture by Lecture ID
func GetLectureById(id primitive.ObjectID) (Lecture, error) {

	var lecture Lecture

	err := collectionLecture.FindOne(context.TODO(), bson.D{{"_id", id}}).Decode(&lecture)

	if err != nil {
		log.New(os.Stdout, "ERROR: ", log.Ltime|log.Lshortfile).Println(err)
		return lecture, err
	}

	log.New(os.Stdout, "INFO: ", log.Ltime|log.Lshortfile).Println("Lecture return success")
	return lecture, nil
}

// UpdateLecture : update lecture with new one
func UpdateLecture(lecture *Lecture) (*mongo.UpdateResult, error) {

	result, err := collectionLecture.ReplaceOne(context.TODO(), bson.D{{"_id", lecture.ID}}, lecture)

	if err != nil {
		log.New(os.Stdout, "ERROR: ", log.Ltime|log.Lshortfile).Println(err)
		return result, err
	}

	log.New(os.Stdout, "INFO: ", log.Ltime|log.Lshortfile).Println("Lecture update success")
	return result, nil
}

// GetAllLectures : return all lectures
func GetAllLectures() ([]Lecture, error) {

	var lectures []Lecture

	cursor, err := collectionLecture.Find(context.TODO(), bson.D{})
	if err != nil {
		log.New(os.Stdout, "ERROR: ", log.Ltime|log.Lshortfile).Println(err)
		return lectures, err
	}

	for cursor.Next(context.TODO()) {
		var lecture Lecture
		err := cursor.Decode(&lecture)
		if err != nil {

			log.New(os.Stdout, "ERROR: ", log.Ltime|log.Lshortfile).Println(err)
			return lectures, err
		}
		lectures = append(lectures, lecture)
	}

	log.New(os.Stdout, "INFO: ", log.Ltime|log.Lshortfile).Println("Lecture return all success")
	return lectures, nil
}

// GetAllLecturesByIds : return lectures by professor ID
func GetAllLecturesByIds(ids []primitive.ObjectID) ([]Lecture, error) {

	var lectures []Lecture

	cursor, err := collectionLecture.Find(context.TODO(), bson.M{"_id": bson.M{"$in": ids}})
	if err != nil {
		log.New(os.Stdout, "ERROR: ", log.Ltime|log.Lshortfile).Println(err)
		return lectures, err
	}

	for cursor.Next(context.TODO()) {
		var lecture Lecture
		err := cursor.Decode(&lecture)
		if err != nil {

			log.New(os.Stdout, "ERROR: ", log.Ltime|log.Lshortfile).Println(err)
			return lectures, err
		}
		lectures = append(lectures, lecture)
	}

	log.New(os.Stdout, "INFO: ", log.Ltime|log.Lshortfile).Println("Lecture return all by professor success")
	return lectures, nil
}

// DeleteLecture : delete lecture
func DeleteLecture(id primitive.ObjectID) (*mongo.DeleteResult, error) {

	result, err := collectionLecture.DeleteOne(context.TODO(), bson.D{{"_id", id}})
	if err != nil {
		log.New(os.Stdout, "ERROR: ", log.Ltime|log.Lshortfile).Println(err)
		return result, err
	}

	log.New(os.Stdout, "INFO: ", log.Ltime|log.Lshortfile).Println("Lecture delete success")
	return result, nil
}
