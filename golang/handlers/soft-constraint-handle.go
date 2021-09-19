package handlers

import (
	"encoding/json"
	"github.com/romana98/NTP/data"
	"github.com/romana98/NTP/enum"
	"go.mongodb.org/mongo-driver/bson/primitive"
	"log"
	"net/http"
	"os"
)

func GetSoftConstraintsByStaff(w http.ResponseWriter, r *http.Request) {

	isAuth := checkToken(r, string(enum.Staff))
	if isAuth != http.StatusOK {
		w.WriteHeader(isAuth)
	}

	email := getLoggedInEmail(r.Header.Get("Authorization"))
	staff, err := data.GetStaffByEmail(email)

	if err != nil {
		log.New(os.Stdout, "ERROR: ", log.Ltime|log.Lshortfile).Println(err)
		w.WriteHeader(http.StatusBadRequest)
		return
	}

	sc, err := data.GetSoftConstraintById(staff.SoftConstraints)

	if err != nil {
		log.New(os.Stdout, "ERROR: ", log.Ltime|log.Lshortfile).Println(err)
		w.WriteHeader(http.StatusBadRequest)
		return
	}

	encoder := json.NewEncoder(w)

	err = encoder.Encode(sc)
	if err != nil {
		log.New(os.Stdout, "ERROR: ", log.Ltime|log.Lshortfile).Println(err)
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
		log.New(os.Stdout, "ERROR: ", log.Ltime|log.Lshortfile).Println(err)
		w.WriteHeader(http.StatusBadRequest)
		return
	}

	newSC := data.NewSoftConstraint(newSCDTO.Prefers)
	newSC.ID, err = primitive.ObjectIDFromHex(newSCDTO.ID)
	if err != nil {
		log.New(os.Stdout, "ERROR: ", log.Ltime|log.Lshortfile).Println(err)
		w.WriteHeader(http.StatusBadRequest)
		return
	}

	_, err = data.UpdateSoftConstraint(newSC)
	if err != nil {
		log.New(os.Stdout, "ERROR: ", log.Ltime|log.Lshortfile).Println(err)
		w.WriteHeader(http.StatusBadRequest)
		return
	}

	encoder := json.NewEncoder(w)
	err = encoder.Encode(newSC)
	if err != nil {
		log.New(os.Stdout, "ERROR: ", log.Ltime|log.Lshortfile).Println(err)
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