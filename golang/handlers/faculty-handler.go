package handlers

import (
	"encoding/json"
	"github.com/gorilla/mux"
	"github.com/romana98/NTP/data"
	"github.com/romana98/NTP/enum"
	"go.mongodb.org/mongo-driver/bson/primitive"
	"log"
	"net/http"
	"os"
)

func GetFaculties(w http.ResponseWriter, r *http.Request) {

	isAuth := checkToken(r, string(enum.Admin))
	if isAuth != http.StatusOK {
		w.WriteHeader(isAuth)
	}

	faculties, err := data.GetAllFaculties()

	if err != nil {
		log.New(os.Stdout, "ERROR: ", log.Ltime|log.Lshortfile).Println(err)
		w.WriteHeader(http.StatusBadRequest)
		return
	}

	encoder := json.NewEncoder(w)

	err = encoder.Encode(faculties)
	if err != nil {
		log.New(os.Stdout, "ERROR: ", log.Ltime|log.Lshortfile).Println(err)
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
		log.New(os.Stdout, "ERROR: ", log.Ltime|log.Lshortfile).Println(err)
		w.WriteHeader(http.StatusBadRequest)
		return
	}

	encoder := json.NewEncoder(w)

	err = encoder.Encode(faculty)
	if err != nil {
		log.New(os.Stdout, "ERROR: ", log.Ltime|log.Lshortfile).Println(err)
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
		log.New(os.Stdout, "ERROR: ", log.Ltime|log.Lshortfile).Println(err)
		w.WriteHeader(http.StatusBadRequest)
		return
	}

	res, err := data.SaveFaculty(&newFaculty)
	if err != nil {
		log.New(os.Stdout, "ERROR: ", log.Ltime|log.Lshortfile).Println(err)
		w.WriteHeader(http.StatusBadRequest)
		return
	}

	faculty, err := data.GetFacultyById(res.InsertedID.(primitive.ObjectID))
	if err != nil {
		log.New(os.Stdout, "ERROR: ", log.Ltime|log.Lshortfile).Println(err)
		w.WriteHeader(http.StatusBadRequest)
		return
	}

	encoder := json.NewEncoder(w)
	err = encoder.Encode(faculty)
	if err != nil {
		log.New(os.Stdout, "ERROR: ", log.Ltime|log.Lshortfile).Println(err)
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
		log.New(os.Stdout, "ERROR: ", log.Ltime|log.Lshortfile).Println(err)
		w.WriteHeader(http.StatusBadRequest)
		return
	}

	objID, err := primitive.ObjectIDFromHex(newFacultyDTO.ID)

	hc, err := primitive.ObjectIDFromHex(newFacultyDTO.HardConstraint)
	schedule, err := primitive.ObjectIDFromHex(newFacultyDTO.Schedule)

	shifts := make([]primitive.ObjectID, 0)
	staff := make([]primitive.ObjectID, 0)
	lectures := make([]primitive.ObjectID, 0)

	for i := 0; i < len(newFacultyDTO.Shifts); i++ {
		s, err := primitive.ObjectIDFromHex(newFacultyDTO.Shifts[i])
		if err != nil {
			log.New(os.Stdout, "ERROR: ", log.Ltime|log.Lshortfile).Println(err)
			w.WriteHeader(http.StatusBadRequest)
			return
		}
		shifts = append(shifts, s)
	}

	for i := 0; i < len(newFacultyDTO.Staff); i++ {
		s, err := primitive.ObjectIDFromHex(newFacultyDTO.Staff[i])
		if err != nil {
			log.New(os.Stdout, "ERROR: ", log.Ltime|log.Lshortfile).Println(err)
			w.WriteHeader(http.StatusBadRequest)
			return
		}
		staff = append(staff, s)
	}

	for i := 0; i < len(newFacultyDTO.Staff); i++ {
		s, err := primitive.ObjectIDFromHex(newFacultyDTO.Staff[i])
		if err != nil {
			log.New(os.Stdout, "ERROR: ", log.Ltime|log.Lshortfile).Println(err)
			w.WriteHeader(http.StatusBadRequest)
			return
		}
		staff = append(staff, s)
		staffUpdate, err := data.GetStaffById(s)
		staffUpdate.Faculty = objID
		_, err = data.UpdateStaff(&staffUpdate)
		if err != nil {
			return
		}
	}

	for i := 0; i < len(newFacultyDTO.Lectures); i++ {
		s, err := primitive.ObjectIDFromHex(newFacultyDTO.Lectures[i])
		if err != nil {
			log.New(os.Stdout, "ERROR: ", log.Ltime|log.Lshortfile).Println(err)
			w.WriteHeader(http.StatusBadRequest)
			return
		}
		lectures = append(lectures, s)
	}

	newFaculty := data.NewFaculty(newFacultyDTO.Name, hc, schedule, shifts, staff, lectures)
	newFaculty.ID = objID
	if err != nil {
		log.New(os.Stdout, "ERROR: ", log.Ltime|log.Lshortfile).Println(err)
		w.WriteHeader(http.StatusBadRequest)
		return
	}

	_, err = data.UpdateFaculty(newFaculty)
	if err != nil {
		log.New(os.Stdout, "ERROR: ", log.Ltime|log.Lshortfile).Println(err)
		w.WriteHeader(http.StatusBadRequest)
		return
	}

	encoder := json.NewEncoder(w)
	err = encoder.Encode(newFaculty)
	if err != nil {
		log.New(os.Stdout, "ERROR: ", log.Ltime|log.Lshortfile).Println(err)
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
		log.New(os.Stdout, "ERROR: ", log.Ltime|log.Lshortfile).Println(err)
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