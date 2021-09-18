package data

import (
	"context"
	"github.com/romana98/NTP/enum"
	"go.mongodb.org/mongo-driver/bson/primitive"
	"os"

	"go.mongodb.org/mongo-driver/bson"
	"go.mongodb.org/mongo-driver/mongo"
	"log"
)

type ShiftMap map[enum.DAY][]Vertex

type ShiftLectureMap map[enum.DAY][]VertexLecture

type Vertex struct {
	Start string `json:"start" bson:"start"`
	End   string `json:"end" bson:"end"`
}

type VertexLecture struct {
	Start string `json:"start" bson:"start"`
	End   string `json:"end" bson:"end"`
	Lecture string `json:"lecture" bson:"lecture"`
}


type AssignedShifts struct {
	Staff  Staff    `json:"staff" bson:"staff"`
	Lectures []Lecture  `json:"lectures" bson:"lectures"`
	Shifts ShiftMap `json:"shifts" bson:"shifts"`
}

type AssignedShiftsByStaffId struct {
	Staff  primitive.ObjectID
	Shifts ShiftLectureMap
}

// Schedule : represents struct for a single Schedule
type Schedule struct {
	ID   primitive.ObjectID        `json:"id" bson:"_id,omitempty"`
	Data []AssignedShiftsByStaffId `json:"data" bson:"data"`
}

func NewSchedule(data []AssignedShiftsByStaffId) *Schedule {
	schedule := new(Schedule)
	schedule.Data = data
	return schedule
}

// SaveSchedule : save new schedule to MongoDB
func SaveSchedule(Schedule *Schedule) (*mongo.InsertOneResult, error) {

	insertResult, err := collectionSchedule.InsertOne(context.TODO(), Schedule)

	if err != nil {
		log.New(os.Stdout, "ERROR: ", log.Ltime|log.Lshortfile).Println(err)
		return insertResult, err
	}

	log.New(os.Stdout, "INFO: ", log.Ltime|log.Lshortfile).Println("Schedule return success")
	return insertResult, nil

}

// GetScheduleById : return schedule by Schedule ID
func GetScheduleById(id primitive.ObjectID) (Schedule, error) {

	var schedule Schedule

	err := collectionSchedule.FindOne(context.TODO(), bson.D{{"_id", id}}).Decode(&schedule)

	if err != nil {
		log.New(os.Stdout, "ERROR: ", log.Ltime|log.Lshortfile).Println(err)
		return schedule, err
	}

	log.New(os.Stdout, "INFO: ", log.Ltime|log.Lshortfile).Println("Schedule return success")
	return schedule, nil
}
