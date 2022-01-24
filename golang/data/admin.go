package data

import (
	"context"
	"github.com/romana98/NTP/enum"
	"github.com/romana98/NTP/logging"
	"go.mongodb.org/mongo-driver/bson"
	"go.mongodb.org/mongo-driver/bson/primitive"
	"go.mongodb.org/mongo-driver/mongo"
)

// Admin : represents struct for a single Admin
type Admin struct {
	ID       primitive.ObjectID `json:"id" bson:"_id,omitempty"`
	Name     string             `json:"name"`
	Surname  string             `json:"surname"`
	Email    string             `json:"email"`
	Password string             `json:"password"`
	Role     enum.ROLE          `json:"role"`
}

func NewAdmin(name, surname, email, password string) *Admin {
	admin := new(Admin)
	admin.Name = name
	admin.Surname = surname
	admin.Email = email
	admin.Password = password
	admin.Role = enum.Admin

	return admin
}

// SaveAdmin : save new admin to MongoDB
func SaveAdmin(admin *Admin) (*mongo.InsertOneResult, error) {

	insertResult, err := collectionAdmin.InsertOne(context.TODO(), admin)
	if err != nil {
		logging.ErrorLogger.Println(err)
		return insertResult, err
	}

	logging.InfoLogger.Println("Admin insert success")
	return insertResult, nil
}

// GetAdminByEmail : return admin by Admin Email
func GetAdminByEmail(email string) (Admin, error) {

	var admin Admin

	err := collectionAdmin.FindOne(context.TODO(), bson.D{{"email", email}}).Decode(&admin)

	if err != nil {
		logging.ErrorLogger.Println(err)
		return admin, err
	}

	logging.InfoLogger.Println("Admin return success")
	return admin, nil
}
