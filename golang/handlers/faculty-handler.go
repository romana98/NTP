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

func GetFaculties(w http.ResponseWriter, r *http.Request) {

	isAuth := checkToken(r, string(enum.Admin))
	if isAuth != http.StatusOK {
		w.WriteHeader(isAuth)
	}

	faculties, err := data.GetAllFaculties()

	if err != nil {
		logging.ErrorLogger.Println(err)
		w.WriteHeader(http.StatusBadRequest)
		return
	}

	encoder := json.NewEncoder(w)

	err = encoder.Encode(faculties)
	if err != nil {
		logging.ErrorLogger.Println(err)
		w.WriteHeader(http.StatusBadRequest)
		return
	}

	w.WriteHeader(http.StatusOK)
	return
}

func GetFaculty(w http.ResponseWriter, r *http.Request) {

	isAuth := checkToken(r, string(enum.Admin))
	if isAuth != http.StatusOK {
		w.WriteHeader(isAuth)
	}

	// Get Post id from url path
	vars := mux.Vars(r)
	id := vars["id"]

	// Transforms string id to PrimitiveId
	idObj, err := primitive.ObjectIDFromHex(id)

	faculty, err := data.GetFacultyById(idObj)

	if err != nil {
		logging.ErrorLogger.Println(err)
		w.WriteHeader(http.StatusBadRequest)
		return
	}

	encoder := json.NewEncoder(w)

	err = encoder.Encode(faculty)
	if err != nil {
		logging.ErrorLogger.Println(err)
		w.WriteHeader(http.StatusBadRequest)
		return
	}

	w.WriteHeader(http.StatusOK)
	return
}

func AddFaculty(w http.ResponseWriter, r *http.Request) {

	isAuth := checkToken(r, string(enum.Admin))
	if isAuth != http.StatusOK {
		w.WriteHeader(isAuth)
		return
	}

	decoder := json.NewDecoder(r.Body)
	var newFaculty data.Faculty
	err := decoder.Decode(&newFaculty)
	if err != nil {
		logging.ErrorLogger.Println(err)
		w.WriteHeader(http.StatusBadRequest)
		return
	}

	res, err := data.SaveFaculty(&newFaculty)
	if err != nil {
		logging.ErrorLogger.Println(err)
		w.WriteHeader(http.StatusBadRequest)
		return
	}

	faculty, err := data.GetFacultyById(res.InsertedID.(primitive.ObjectID))
	if err != nil {
		logging.ErrorLogger.Println(err)
		w.WriteHeader(http.StatusBadRequest)
		return
	}

	encoder := json.NewEncoder(w)
	err = encoder.Encode(faculty)
	if err != nil {
		logging.ErrorLogger.Println(err)
		w.WriteHeader(http.StatusBadRequest)
		return
	}

	w.WriteHeader(http.StatusOK)
}

func UpdateFaculty(w http.ResponseWriter, r *http.Request) {

	isAuth := checkToken(r, string(enum.Admin))
	if isAuth != http.StatusOK {
		w.WriteHeader(isAuth)
		return
	}

	decoder := json.NewDecoder(r.Body)
	var newFacultyDTO FacultyDTO
	err := decoder.Decode(&newFacultyDTO)
	if err != nil {
		logging.ErrorLogger.Println(err)
		w.WriteHeader(http.StatusBadRequest)
		return
	}

	objID, err := primitive.ObjectIDFromHex(newFacultyDTO.ID)

	hc, err := primitive.ObjectIDFromHex(newFacultyDTO.HardConstraint)
	schedule, err := primitive.ObjectIDFromHex(newFacultyDTO.Schedule)

	shifts := make([]primitive.ObjectID, 0)
	lectures := make([]primitive.ObjectID, 0)

	for i := 0; i < len(newFacultyDTO.Shifts); i++ {
		s, err := primitive.ObjectIDFromHex(newFacultyDTO.Shifts[i])
		if err != nil {
			logging.ErrorLogger.Println(err)
			w.WriteHeader(http.StatusBadRequest)
			return
		}
		shifts = append(shifts, s)
	}

	for i := 0; i < len(newFacultyDTO.Lectures); i++ {
		s, err := primitive.ObjectIDFromHex(newFacultyDTO.Lectures[i])
		if err != nil {
			logging.ErrorLogger.Println(err)
			w.WriteHeader(http.StatusBadRequest)
			return
		}
		lectures = append(lectures, s)
	}

	faculty, err := data.GetFacultyById(objID)

	if err != nil {
		logging.ErrorLogger.Println(err)
		w.WriteHeader(http.StatusBadRequest)
		return
	}

	newFaculty := data.NewFaculty(newFacultyDTO.Name, hc, schedule, shifts, faculty.Staff, lectures)
	newFaculty.ID = objID
	if err != nil {
		logging.ErrorLogger.Println(err)
		w.WriteHeader(http.StatusBadRequest)
		return
	}

	_, err = data.UpdateFaculty(newFaculty)
	if err != nil {
		logging.ErrorLogger.Println(err)
		w.WriteHeader(http.StatusBadRequest)
		return
	}

	encoder := json.NewEncoder(w)
	err = encoder.Encode(newFaculty)
	if err != nil {
		logging.ErrorLogger.Println(err)
		w.WriteHeader(http.StatusBadRequest)
		return
	}

	w.WriteHeader(http.StatusOK)
}

func DeleteFaculty(w http.ResponseWriter, r *http.Request) {

	isAuth := checkToken(r, string(enum.Admin))
	if isAuth != http.StatusOK {
		w.WriteHeader(isAuth)
	}

	// Get Post id from url path
	vars := mux.Vars(r)
	id := vars["id"]

	// Transforms string id to PrimitiveId
	idObj, err := primitive.ObjectIDFromHex(id)

	_, err = data.DeleteFaculty(idObj)

	if err != nil {
		logging.ErrorLogger.Println(err)
		w.WriteHeader(http.StatusBadRequest)
		return
	}

	w.WriteHeader(http.StatusOK)
	return
}

// FacultyDTO : represents struct for a single FacultyDTO
type FacultyDTO struct {
	ID             string   `json:"id" bson:"_id,omitempty"`
	Name           string   `json:"name"`
	HardConstraint string   `json:"hard_constraint"`
	Shifts         []string `json:"shifts"`
	Staff          []string `json:"staff"`
	Lectures       []string `json:"lectures"`
	Schedule       string   `json:"schedule"`
}
