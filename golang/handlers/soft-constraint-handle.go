package handlers

import (
	"encoding/json"
	"github.com/romana98/NTP/data"
	"github.com/romana98/NTP/enum"
	"github.com/romana98/NTP/logging"
	"go.mongodb.org/mongo-driver/bson/primitive"
	"net/http"
)

func GetSoftConstraintsByStaff(w http.ResponseWriter, r *http.Request) {

	isAuth := checkToken(r, string(enum.Staff))
	if isAuth != http.StatusOK {
		w.WriteHeader(isAuth)
	}

	tokenString := r.Header.Get("Authorization")
	email := getLoggedInEmail(tokenString[7:])
	staff, err := data.GetStaffByEmail(email)

	if err != nil {
		logging.ErrorLogger.Println(err)
		w.WriteHeader(http.StatusBadRequest)
		return
	}

	sc, err := data.GetSoftConstraintById(staff.SoftConstraints)

	if err != nil {
		logging.ErrorLogger.Println(err)
		w.WriteHeader(http.StatusBadRequest)
		return
	}

	encoder := json.NewEncoder(w)

	err = encoder.Encode(sc)
	if err != nil {
		logging.ErrorLogger.Println(err)
		w.WriteHeader(http.StatusBadRequest)
		return
	}

	w.WriteHeader(http.StatusOK)
	return
}

func UpdateSoftConstraint(w http.ResponseWriter, r *http.Request) {

	isAuth := checkToken(r, string(enum.Staff))
	if isAuth != http.StatusOK {
		w.WriteHeader(isAuth)
		return
	}

	decoder := json.NewDecoder(r.Body)
	var newSCDTO SoftConstraintDTO
	err := decoder.Decode(&newSCDTO)
	if err != nil {
		logging.ErrorLogger.Println(err)
		w.WriteHeader(http.StatusBadRequest)
		return
	}

	newSC := data.NewSoftConstraint(newSCDTO.Prefers)
	newSC.ID, err = primitive.ObjectIDFromHex(newSCDTO.ID)
	if err != nil {
		logging.ErrorLogger.Println(err)
		w.WriteHeader(http.StatusBadRequest)
		return
	}

	_, err = data.UpdateSoftConstraint(newSC)
	if err != nil {
		logging.ErrorLogger.Println(err)
		w.WriteHeader(http.StatusBadRequest)
		return
	}

	encoder := json.NewEncoder(w)
	err = encoder.Encode(newSC)
	if err != nil {
		logging.ErrorLogger.Println(err)
		w.WriteHeader(http.StatusBadRequest)
		return
	}

	w.WriteHeader(http.StatusOK)
}

// SoftConstraintDTO : represents struct for a single SoftConstraintDTO
type SoftConstraintDTO struct {
	ID      string           `json:"id" bson:"_id,omitempty"`
	Prefers map[enum.DAY]int `json:"prefers"`
}
