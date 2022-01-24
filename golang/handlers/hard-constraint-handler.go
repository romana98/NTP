package handlers

import (
	"encoding/json"
	"github.com/gorilla/mux"
	"github.com/romana98/NTP/data"
	"github.com/romana98/NTP/enum"
	"github.com/romana98/NTP/logging"
	"go.mongodb.org/mongo-driver/bson/primitive"
	"net/http"
)

func GetHardConstraints(w http.ResponseWriter, r *http.Request) {

	isAuth := checkToken(r, string(enum.Admin))
	if isAuth != http.StatusOK {
		w.WriteHeader(isAuth)
	}

	hcs, err := data.GetAllHardConstraints()

	if err != nil {
		logging.ErrorLogger.Println(err)
		w.WriteHeader(http.StatusBadRequest)
		return
	}

	encoder := json.NewEncoder(w)

	err = encoder.Encode(hcs)
	if err != nil {
		logging.ErrorLogger.Println(err)
		w.WriteHeader(http.StatusBadRequest)
		return
	}

	w.WriteHeader(http.StatusOK)
	return
}

func GetHardConstraint(w http.ResponseWriter, r *http.Request) {

	isAuth := checkToken(r, string(enum.Admin))
	if isAuth != http.StatusOK {
		w.WriteHeader(isAuth)
	}

	// Get Post id from url path
	vars := mux.Vars(r)
	id := vars["id"]

	// Transforms string id to PrimitiveId
	idObj, err := primitive.ObjectIDFromHex(id)

	hc, err := data.GetHardConstraintById(idObj)

	if err != nil {
		logging.ErrorLogger.Println(err)
		w.WriteHeader(http.StatusBadRequest)
		return
	}

	encoder := json.NewEncoder(w)

	err = encoder.Encode(hc)
	if err != nil {
		logging.ErrorLogger.Println(err)
		w.WriteHeader(http.StatusBadRequest)
		return
	}

	w.WriteHeader(http.StatusOK)
	return
}

func AddHardConstraint(w http.ResponseWriter, r *http.Request) {

	isAuth := checkToken(r, string(enum.Admin))
	if isAuth != http.StatusOK {
		w.WriteHeader(isAuth)
		return
	}

	decoder := json.NewDecoder(r.Body)
	var newHardConstraint data.HardConstraint
	err := decoder.Decode(&newHardConstraint)
	if err != nil {
		logging.ErrorLogger.Println(err)
		w.WriteHeader(http.StatusBadRequest)
		return
	}

	res, err := data.SaveHardConstraint(&newHardConstraint)
	if err != nil {
		logging.ErrorLogger.Println(err)
		w.WriteHeader(http.StatusBadRequest)
		return
	}

	hc, err := data.GetHardConstraintById(res.InsertedID.(primitive.ObjectID))
	if err != nil {
		logging.ErrorLogger.Println(err)
		w.WriteHeader(http.StatusBadRequest)
		return
	}

	encoder := json.NewEncoder(w)
	err = encoder.Encode(hc)
	if err != nil {
		logging.ErrorLogger.Println(err)
		w.WriteHeader(http.StatusBadRequest)
		return
	}

	w.WriteHeader(http.StatusOK)
}

func UpdateHardConstraint(w http.ResponseWriter, r *http.Request) {

	isAuth := checkToken(r, string(enum.Admin))
	if isAuth != http.StatusOK {
		w.WriteHeader(isAuth)
		return
	}

	decoder := json.NewDecoder(r.Body)
	var newHardConstraintDTO HardConstraintDTO
	err := decoder.Decode(&newHardConstraintDTO)
	if err != nil {
		logging.ErrorLogger.Println(err)
		w.WriteHeader(http.StatusBadRequest)
		return
	}

	newHardConstraint := data.NewHardConstraint(newHardConstraintDTO.DailyMax, newHardConstraintDTO.WeeklyMax, newHardConstraintDTO.WeeklyMin, newHardConstraintDTO.MaxPerShift)
	newHardConstraint.ID, err = primitive.ObjectIDFromHex(newHardConstraintDTO.ID)
	if err != nil {
		logging.ErrorLogger.Println(err)
		w.WriteHeader(http.StatusBadRequest)
		return
	}

	_, err = data.UpdateHardConstraint(newHardConstraint)
	if err != nil {
		logging.ErrorLogger.Println(err)
		w.WriteHeader(http.StatusBadRequest)
		return
	}

	encoder := json.NewEncoder(w)
	err = encoder.Encode(newHardConstraint)
	if err != nil {
		logging.ErrorLogger.Println(err)
		w.WriteHeader(http.StatusBadRequest)
		return
	}

	w.WriteHeader(http.StatusOK)
}

func DeleteHardConstraint(w http.ResponseWriter, r *http.Request) {

	isAuth := checkToken(r, string(enum.Admin))
	if isAuth != http.StatusOK {
		w.WriteHeader(isAuth)
	}

	// Get Post id from url path
	vars := mux.Vars(r)
	id := vars["id"]

	// Transforms string id to PrimitiveId
	idObj, err := primitive.ObjectIDFromHex(id)

	_, err = data.DeleteHardConstraint(idObj)

	if err != nil {
		logging.ErrorLogger.Println(err)
		w.WriteHeader(http.StatusBadRequest)
		return
	}

	w.WriteHeader(http.StatusOK)
	return
}

// HardConstraintDTO : represents struct for a single HardConstraintDTO
type HardConstraintDTO struct {
	ID          string `json:"id" bson:"_id,omitempty"`
	DailyMax    int    `json:"daily_max"`
	WeeklyMax   int    `json:"weekly_max"`
	WeeklyMin   int    `json:"weekly_min"`
	MaxPerShift int    `json:"max_per_shift"`
}
