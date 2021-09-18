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

// Staff : represents struct for a single Staff
type Staff struct {
	ID              primitive.ObjectID   `json:"id" bson:"_id,omitempty"`
	Name            string               `json:"name"`
	Surname         string               `json:"surname"`
	Email           string               `json:"email"`
	Password        string               `json:"password"`
	Lectures        []primitive.ObjectID `json:"lectures"`
	SoftConstraints primitive.ObjectID   `json:"soft_constraints"`
	Faculty         primitive.ObjectID   `json:"faculty"`
	Role            enum.ROLE            `json:"role"`
}

func NewStaff(name, surname, email string, lectures []primitive.ObjectID, sc, faculty primitive.ObjectID) *Staff {
	staff := new(Staff)
	staff.Name = name
	staff.Surname = surname
	staff.Email = email
	staff.Password = "123qweasd"
	staff.Lectures = lectures
	staff.SoftConstraints = sc
	staff.Faculty = faculty
	staff.Role = enum.Staff

	return staff
}

// SaveStaff : save new staff to MongoDB
func SaveStaff(staff *Staff) (*mongo.InsertOneResult, error) {

	insertResult, err := collectionStaff.InsertOne(context.TODO(), staff)
	if err != nil {
		log.New(os.Stdout, "ERROR: ", log.Ltime|log.Lshortfile).Println(err)
		return insertResult, err
	}

	log.New(os.Stdout, "INFO: ", log.Ltime|log.Lshortfile).Println("Staff insert success")
	return insertResult, nil

}

// GetStaffById : return staff by Staff ID
func GetStaffById(id primitive.ObjectID) (Staff, error) {

	var staff Staff

	err := collectionStaff.FindOne(context.TODO(), bson.D{{"_id", id}}).Decode(&staff)

	if err != nil {
		log.New(os.Stdout, "ERROR: ", log.Ltime|log.Lshortfile).Println(err)
		return staff, err
	}

	log.New(os.Stdout, "INFO: ", log.Ltime|log.Lshortfile).Println("Staff return success")
	return staff, nil
}

// GetStaffByEmail : return admin by Staff Email
func GetStaffByEmail(email string) (Staff, error) {

	var staff Staff

	err := collectionStaff.FindOne(context.TODO(), bson.D{{"email", email}}).Decode(&staff)

	if err != nil {
		log.New(os.Stdout, "ERROR: ", log.Ltime|log.Lshortfile).Println(err)
		return staff, err
	}

	log.New(os.Stdout, "INFO: ", log.Ltime|log.Lshortfile).Println("Staff return success")
	return staff, nil
}

// UpdateStaff : update staff with new one
func UpdateStaff(staff *Staff) (*mongo.UpdateResult, error) {

	result, err := collectionStaff.ReplaceOne(context.TODO(), bson.D{{"_id", staff.ID}}, staff)

	if err != nil {
		log.New(os.Stdout, "ERROR: ", log.Ltime|log.Lshortfile).Println(err)
		return result, err
	}

	log.New(os.Stdout, "INFO: ", log.Ltime|log.Lshortfile).Println("Staff update success")
	return result, nil
}

// GetAllStaff : return all staffList
func GetAllStaff() ([]Staff, error) {

	var staffList []Staff

	cursor, err := collectionStaff.Find(context.TODO(), bson.D{})
	if err != nil {
		log.New(os.Stdout, "ERROR: ", log.Ltime|log.Lshortfile).Println(err)
		return staffList, err
	}

	for cursor.Next(context.TODO()) {
		var staff Staff
		err := cursor.Decode(&staff)
		if err != nil {

			log.New(os.Stdout, "ERROR: ", log.Ltime|log.Lshortfile).Println(err)
			return staffList, err
		}
		staffList = append(staffList, staff)
	}

	log.New(os.Stdout, "INFO: ", log.Ltime|log.Lshortfile).Println("Staff return all success")
	return staffList, nil
}

// GetAllStaffByFaculty : return all by faculty
func GetAllStaffByFaculty(ids []primitive.ObjectID) ([]Staff, error) {

	var staffList []Staff

	cursor, err := collectionStaff.Find(context.TODO(), bson.M{"_id": bson.M{"$in": ids}})
	if err != nil {
		log.New(os.Stdout, "ERROR: ", log.Ltime|log.Lshortfile).Println(err)
		return staffList, err
	}

	for cursor.Next(context.TODO()) {
		var staff Staff
		err := cursor.Decode(&staff)
		if err != nil {

			log.New(os.Stdout, "ERROR: ", log.Ltime|log.Lshortfile).Println(err)
			return staffList, err
		}
		staffList = append(staffList, staff)

	}
	log.New(os.Stdout, "INFO: ", log.Ltime|log.Lshortfile).Println("Staff return all by faculty success")
	return staffList, nil
}

// DeleteStaff : delete staff
func DeleteStaff(id primitive.ObjectID) (*mongo.DeleteResult, error) {

	result, err := collectionStaff.DeleteOne(context.TODO(), bson.D{{"_id", id}})
	if err != nil {
		log.New(os.Stdout, "ERROR: ", log.Ltime|log.Lshortfile).Println(err)
		return result, err
	}

	log.New(os.Stdout, "INFO: ", log.Ltime|log.Lshortfile).Println("Staff delete success")
	return result, nil
}
