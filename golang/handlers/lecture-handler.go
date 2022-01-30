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

func GetLectures(w http.ResponseWriter, r *http.Request) {

	isAuth := checkToken(r, string(enum.Admin))
	if isAuth != http.StatusOK {
		w.WriteHeader(isAuth)
	}

	lectures, err := data.GetAllLectures()

	if err != nil {
		logging.ErrorLogger.Println(err)
		w.WriteHeader(http.StatusBadRequest)
		return
	}

	encoder := json.NewEncoder(w)

	err = encoder.Encode(lectures)
	if err != nil {
		logging.ErrorLogger.Println(err)
		w.WriteHeader(http.StatusBadRequest)
		return
	}

	w.WriteHeader(http.StatusOK)
	return
}

func GetLecture(w http.ResponseWriter, r *http.Request) {

	isAuth := checkToken(r, string(enum.Admin))
	if isAuth != http.StatusOK {
		w.WriteHeader(isAuth)
	}

	// Get Post id from url path
	vars := mux.Vars(r)
	id := vars["id"]

	// Transforms string id to PrimitiveId
	idObj, err := primitive.ObjectIDFromHex(id)

	lecture, err := data.GetLectureById(idObj)

	if err != nil {
		logging.ErrorLogger.Println(err)
		w.WriteHeader(http.StatusBadRequest)
		return
	}

	encoder := json.NewEncoder(w)

	err = encoder.Encode(lecture)
	if err != nil {
		logging.ErrorLogger.Println(err)
		w.WriteHeader(http.StatusBadRequest)
		return
	}

	w.WriteHeader(http.StatusOK)
	return
}

func AddLecture(w http.ResponseWriter, r *http.Request) {

	isAuth := checkToken(r, string(enum.Admin))
	if isAuth != http.StatusOK {
		w.WriteHeader(isAuth)
		return
	}

	decoder := json.NewDecoder(r.Body)
	var newLecture data.Lecture
	err := decoder.Decode(&newLecture)
	if err != nil {
		logging.ErrorLogger.Println(err)
		w.WriteHeader(http.StatusBadRequest)
		return
	}

	res, err := data.SaveLecture(&newLecture)
	if err != nil {
		logging.ErrorLogger.Println(err)
		w.WriteHeader(http.StatusBadRequest)
		return
	}

	lecture, err := data.GetLectureById(res.InsertedID.(primitive.ObjectID))
	if err != nil {
		logging.ErrorLogger.Println(err)
		w.WriteHeader(http.StatusBadRequest)
		return
	}

	encoder := json.NewEncoder(w)
	err = encoder.Encode(lecture)
	if err != nil {
		logging.ErrorLogger.Println(err)
		w.WriteHeader(http.StatusBadRequest)
		return
	}

	w.WriteHeader(http.StatusOK)
}

func UpdateLecture(w http.ResponseWriter, r *http.Request) {

	isAuth := checkToken(r, string(enum.Admin))
	if isAuth != http.StatusOK {
		w.WriteHeader(isAuth)
		return
	}

	decoder := json.NewDecoder(r.Body)
	var newLectureDTO LectureDTO
	err := decoder.Decode(&newLectureDTO)
	if err != nil {
		logging.ErrorLogger.Println(err)
		w.WriteHeader(http.StatusBadRequest)
		return
	}

	newLecture := data.NewLecture(newLectureDTO.NumberOfTimes, newLectureDTO.Name)
	newLecture.ID, err = primitive.ObjectIDFromHex(newLectureDTO.ID)
	if err != nil {
		logging.ErrorLogger.Println(err)
		w.WriteHeader(http.StatusBadRequest)
		return
	}

	_, err = data.UpdateLecture(newLecture)
	if err != nil {
		logging.ErrorLogger.Println(err)
		w.WriteHeader(http.StatusBadRequest)
		return
	}

	encoder := json.NewEncoder(w)
	err = encoder.Encode(newLecture)
	if err != nil {
		logging.ErrorLogger.Println(err)
		w.WriteHeader(http.StatusBadRequest)
		return
	}

	w.WriteHeader(http.StatusOK)
}

func DeleteLecture(w http.ResponseWriter, r *http.Request) {

	isAuth := checkToken(r, string(enum.Admin))
	if isAuth != http.StatusOK {
		w.WriteHeader(isAuth)
	}

	// Get Post id from url path
	vars := mux.Vars(r)
	id := vars["id"]

	// Transforms string id to PrimitiveId
	idObj, err := primitive.ObjectIDFromHex(id)

	_, err = data.DeleteLecture(idObj)

	if err != nil {
		logging.ErrorLogger.Println(err)
		w.WriteHeader(http.StatusBadRequest)
		return
	}

	w.WriteHeader(http.StatusOK)
	return
}

func GetLecturesByStaff(w http.ResponseWriter, r *http.Request) {

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

	lectures, err := data.GetAllLecturesByIds(staff.Lectures)

	if err != nil {
		logging.ErrorLogger.Println(err)
		w.WriteHeader(http.StatusBadRequest)
		return
	}

	encoder := json.NewEncoder(w)

	err = encoder.Encode(lectures)
	if err != nil {
		logging.ErrorLogger.Println(err)
		w.WriteHeader(http.StatusBadRequest)
		return
	}

	w.WriteHeader(http.StatusOK)
	return
}

func GetLecturesByIds(w http.ResponseWriter, r *http.Request) {

	isAuth := checkToken(r, string(enum.Admin))
	if isAuth != http.StatusOK {
		w.WriteHeader(isAuth)
	}

	decoder := json.NewDecoder(r.Body)
	var lectureIDs []string

	err := decoder.Decode(&lectureIDs)
	if err != nil {
		logging.ErrorLogger.Println(err)
		w.WriteHeader(http.StatusBadRequest)
		return
	}

	lectureIDsPrim := make([]primitive.ObjectID, 0)
	for i := 0; i < len(lectureIDs); i++ {
		id, _ := primitive.ObjectIDFromHex(lectureIDs[i])
		lectureIDsPrim = append(lectureIDsPrim, id)
	}

	lectures, err := data.GetAllLecturesByIds(lectureIDsPrim)

	if err != nil {
		logging.ErrorLogger.Println(err)
		w.WriteHeader(http.StatusBadRequest)
		return
	}

	encoder := json.NewEncoder(w)

	err = encoder.Encode(lectures)
	if err != nil {
		logging.ErrorLogger.Println(err)
		w.WriteHeader(http.StatusBadRequest)
		return
	}

	w.WriteHeader(http.StatusOK)
	return
}

// LectureDTO : represents struct for a single LectureDTO
type LectureDTO struct {
	ID            string `json:"id" bson:"_id,omitempty"`
	Name          string `json:"name"`
	NumberOfTimes int    `json:"number_of_times"`
}
