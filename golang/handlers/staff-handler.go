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

func GetStaffList(w http.ResponseWriter, r *http.Request) {

	isAuth := checkToken(r, string(enum.Admin))
	if isAuth != http.StatusOK {
		w.WriteHeader(isAuth)
	}

	staffList, err := data.GetAllStaff()

	if err != nil {
		log.New(os.Stdout, "ERROR: ", log.Ltime|log.Lshortfile).Println(err)
		w.WriteHeader(http.StatusBadRequest)
		return
	}

	encoder := json.NewEncoder(w)

	err = encoder.Encode(staffList)
	if err != nil {
		log.New(os.Stdout, "ERROR: ", log.Ltime|log.Lshortfile).Println(err)
		w.WriteHeader(http.StatusBadRequest)
		return
	}

	w.WriteHeader(http.StatusOK)
	return
}

func GetStaff(w http.ResponseWriter, r *http.Request) {

	isAuth := checkToken(r, string(enum.Admin))
	if isAuth != http.StatusOK {
		w.WriteHeader(isAuth)
	}

	// Get Post id from url path
	vars := mux.Vars(r)
	id := vars["id"]

	// Transforms string id to PrimitiveId
	idObj, err := primitive.ObjectIDFromHex(id)

	staff, err := data.GetStaffById(idObj)

	if err != nil {
		log.New(os.Stdout, "ERROR: ", log.Ltime|log.Lshortfile).Println(err)
		w.WriteHeader(http.StatusBadRequest)
		return
	}

	encoder := json.NewEncoder(w)

	err = encoder.Encode(staff)
	if err != nil {
		log.New(os.Stdout, "ERROR: ", log.Ltime|log.Lshortfile).Println(err)
		w.WriteHeader(http.StatusBadRequest)
		return
	}

	w.WriteHeader(http.StatusOK)
	return
}

func AddStaff(w http.ResponseWriter, r *http.Request) {

	isAuth := checkToken(r, string(enum.Admin))
	if isAuth != http.StatusOK {
		w.WriteHeader(isAuth)
		return
	}

	decoder := json.NewDecoder(r.Body)
	var newStaffDTO StaffDTO
	err := decoder.Decode(&newStaffDTO)
	if err != nil {
		log.New(os.Stdout, "ERROR: ", log.Ltime|log.Lshortfile).Println(err)
		w.WriteHeader(http.StatusBadRequest)
		return
	}

	lectures := make([]primitive.ObjectID, 0)

	for i := 0; i < len(newStaffDTO.Lectures); i++ {
		l, _ := primitive.ObjectIDFromHex(newStaffDTO.Lectures[i])
		lectures = append(lectures, l)
	}

	facultyId, _ := primitive.ObjectIDFromHex(newStaffDTO.Faculty)

	sc := data.NewSoftConstraint(generateSoftConstraint())

	res, _ := data.SaveSoftConstraint(sc)

	newStaff := data.NewStaff(newStaffDTO.Name, newStaffDTO.Surname, newStaffDTO.Email, lectures, res.InsertedID.(primitive.ObjectID), facultyId)

	res, err = data.SaveStaff(newStaff)
	if err != nil {
		log.New(os.Stdout, "ERROR: ", log.Ltime|log.Lshortfile).Println(err)
		w.WriteHeader(http.StatusBadRequest)
		return
	}

	faculty, err := data.GetFacultyById(facultyId)
	if err != nil {
		log.New(os.Stdout, "ERROR: ", log.Ltime|log.Lshortfile).Println(err)
		w.WriteHeader(http.StatusBadRequest)
		return
	}

	faculty.Staff = append(faculty.Staff, res.InsertedID.(primitive.ObjectID))
	_, err = data.UpdateFaculty(&faculty)
	if err != nil {
		log.New(os.Stdout, "ERROR: ", log.Ltime|log.Lshortfile).Println(err)
		w.WriteHeader(http.StatusBadRequest)
		return
	}

	staff, err := data.GetStaffById(res.InsertedID.(primitive.ObjectID))
	if err != nil {
		log.New(os.Stdout, "ERROR: ", log.Ltime|log.Lshortfile).Println(err)
		w.WriteHeader(http.StatusBadRequest)
		return
	}

	encoder := json.NewEncoder(w)
	err = encoder.Encode(staff)
	if err != nil {
		log.New(os.Stdout, "ERROR: ", log.Ltime|log.Lshortfile).Println(err)
		w.WriteHeader(http.StatusBadRequest)
		return
	}

	w.WriteHeader(http.StatusOK)
}

func UpdateStaff(w http.ResponseWriter, r *http.Request) {

	isAuth := checkToken(r, string(enum.Admin))
	if isAuth != http.StatusOK {
		w.WriteHeader(isAuth)
		return
	}

	decoder := json.NewDecoder(r.Body)
	var newStaffDTO StaffDTO
	err := decoder.Decode(&newStaffDTO)
	if err != nil {
		log.New(os.Stdout, "ERROR: ", log.Ltime|log.Lshortfile).Println(err)
		w.WriteHeader(http.StatusBadRequest)
		return
	}

	lectures := make([]primitive.ObjectID, 0)

	for i := 0; i < len(newStaffDTO.Lectures); i++ {
		l, _ := primitive.ObjectIDFromHex(newStaffDTO.Lectures[i])
		lectures = append(lectures, l)
	}

	faculty, _ := primitive.ObjectIDFromHex(newStaffDTO.Faculty)

	staffID, err := primitive.ObjectIDFromHex(newStaffDTO.ID)
	if err != nil {
		log.New(os.Stdout, "ERROR: ", log.Ltime|log.Lshortfile).Println(err)
		w.WriteHeader(http.StatusBadRequest)
		return
	}
	staff, err := data.GetStaffById(staffID)
	if err != nil {
		log.New(os.Stdout, "ERROR: ", log.Ltime|log.Lshortfile).Println(err)
		w.WriteHeader(http.StatusBadRequest)
		return
	}

	newStaff := data.NewStaff(newStaffDTO.Name, newStaffDTO.Surname, newStaffDTO.Email, lectures, staff.SoftConstraints, faculty)
	newStaff.ID = staffID
	if err != nil {
		log.New(os.Stdout, "ERROR: ", log.Ltime|log.Lshortfile).Println(err)
		w.WriteHeader(http.StatusBadRequest)
		return
	}

	_, err = data.UpdateStaff(newStaff)
	if err != nil {
		log.New(os.Stdout, "ERROR: ", log.Ltime|log.Lshortfile).Println(err)
		w.WriteHeader(http.StatusBadRequest)
		return
	}

	encoder := json.NewEncoder(w)
	err = encoder.Encode(newStaff)
	if err != nil {
		log.New(os.Stdout, "ERROR: ", log.Ltime|log.Lshortfile).Println(err)
		w.WriteHeader(http.StatusBadRequest)
		return
	}

	w.WriteHeader(http.StatusOK)
}

func DeleteStaff(w http.ResponseWriter, r *http.Request) {

	isAuth := checkToken(r, string(enum.Admin))
	if isAuth != http.StatusOK {
		w.WriteHeader(isAuth)
	}

	// Get Post id from url path
	vars := mux.Vars(r)
	id := vars["id"]

	// Transforms string id to PrimitiveId
	idObj, err := primitive.ObjectIDFromHex(id)

	_, err = data.DeleteStaff(idObj)

	if err != nil {
		log.New(os.Stdout, "ERROR: ", log.Ltime|log.Lshortfile).Println(err)
		w.WriteHeader(http.StatusBadRequest)
		return
	}

	w.WriteHeader(http.StatusOK)
	return
}

// StaffDTO : represents struct for a single StaffDTO
type StaffDTO struct {
	ID       string   `json:"id" bson:"_id,omitempty"`
	Name     string   `json:"name"`
	Surname  string   `json:"surname"`
	Email    string   `json:"email"`
	Lectures []string `json:"lectures"`
	Faculty  string   `json:"faculty"`
}

func generateSoftConstraint() map[enum.DAY]int {
	sc := make(map[enum.DAY]int, 0)

	sc[enum.Monday] = 1
	sc[enum.Tuesday] = 1
	sc[enum.Wednesday] = 1
	sc[enum.Thursday] = 1
	sc[enum.Friday] = 1

	return sc
}
