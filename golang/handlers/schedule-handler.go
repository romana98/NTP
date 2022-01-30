package handlers

import (
	"encoding/json"
	"github.com/gorilla/mux"
	"github.com/romana98/NTP/algorithm"
	"github.com/romana98/NTP/data"
	"github.com/romana98/NTP/enum"
	"github.com/romana98/NTP/logging"
	"go.mongodb.org/mongo-driver/bson/primitive"
	"net/http"
)

func GenerateSchedule(w http.ResponseWriter, r *http.Request) {
	isAuth := checkToken(r, string(enum.Admin))
	if isAuth != http.StatusOK {
		w.WriteHeader(isAuth)
	}

	decoder := json.NewDecoder(r.Body)
	var ID IdDTO
	err := decoder.Decode(&ID)
	if err != nil {
		logging.ErrorLogger.Println(err)
		w.WriteHeader(http.StatusBadRequest)
		return
	}
	// Transforms string id to PrimitiveId
	idObj, err := primitive.ObjectIDFromHex(ID.ID)

	if err != nil {
		logging.ErrorLogger.Println(err)
		w.WriteHeader(http.StatusBadRequest)
		return

	}

	faculty, err := data.GetFacultyById(idObj)

	logging.InfoLogger.Println("Algorithm started")
	algorithm.Start(&faculty)

	if err != nil {
		logging.ErrorLogger.Println(err)
		w.WriteHeader(http.StatusBadRequest)
		return

	}

	schedule, err := data.GetScheduleById(faculty.Schedule)

	if err != nil {
		logging.ErrorLogger.Println(err)
		w.WriteHeader(http.StatusBadRequest)
		return

	}

	scheduleDTOList := make([]ScheduleDTO, 0)
	for _, el := range schedule.Data {
		staff, err := data.GetStaffById(el.Staff)
		if err != nil {
			logging.ErrorLogger.Println(err)
			w.WriteHeader(http.StatusBadRequest)
			return
		}
		for day, shifts := range el.Shifts {
			for _, shift := range shifts {
				scheduleDTOList = append(scheduleDTOList, ScheduleDTO{staff.Name + " " + staff.Surname + ", " + staff.Email, shift.Lecture,string(day), shift.Start + " - " + shift.End})
			}
		}
	}

	encoder := json.NewEncoder(w)

	err = encoder.Encode(scheduleDTOList)
	if err != nil {
		logging.ErrorLogger.Println(err)
		w.WriteHeader(http.StatusBadRequest)
		return
	}

	w.WriteHeader(http.StatusOK)
	return

}

func GetSchedule(w http.ResponseWriter, r *http.Request) {
	isAuth := checkToken(r, string(enum.Admin))
	if isAuth != http.StatusOK {
		w.WriteHeader(isAuth)
	}

	// Get Post id from url path
	vars := mux.Vars(r)
	id := vars["id"]

	// Transforms string id to PrimitiveId
	idObj, err := primitive.ObjectIDFromHex(id)

	if err != nil {
		logging.ErrorLogger.Println(err)
		w.WriteHeader(http.StatusBadRequest)
		return

	}

	schedule, err := data.GetScheduleById(idObj)

	if err != nil {
		logging.ErrorLogger.Println(err)
		w.WriteHeader(http.StatusBadRequest)
		return

	}

	scheduleDTOList := make([]ScheduleDTO, 0)
	for _, el := range schedule.Data {
		staff, err := data.GetStaffById(el.Staff)
		if err != nil {
			logging.ErrorLogger.Println(err)
			w.WriteHeader(http.StatusBadRequest)
			return
		}
		for day, shifts := range el.Shifts {
			for _, shift := range shifts {
				scheduleDTOList = append(scheduleDTOList, ScheduleDTO{staff.Name + " " + staff.Surname + ", " + staff.Email, shift.Lecture,string(day), shift.Start + " - " + shift.End})
			}
		}
	}

	encoder := json.NewEncoder(w)

	err = encoder.Encode(scheduleDTOList)
	if err != nil {
		logging.ErrorLogger.Println(err)
		w.WriteHeader(http.StatusBadRequest)
		return
	}

	w.WriteHeader(http.StatusOK)
	return
}

func GetScheduleByStaff(w http.ResponseWriter, r *http.Request) {
	isAuth := checkToken(r, string(enum.Staff))
	if isAuth != http.StatusOK {
		w.WriteHeader(isAuth)
	}

	tokenString := r.Header.Get("Authorization")
	email := getLoggedInEmail(tokenString[7:])
	staffLoggedIn, err := data.GetStaffByEmail(email)

	if err != nil {
		logging.ErrorLogger.Println(err)
		w.WriteHeader(http.StatusBadRequest)
		return
	}

	faculty, err := data.GetFacultyById(staffLoggedIn.Faculty)

	if err != nil {
		logging.ErrorLogger.Println(err)
		w.WriteHeader(http.StatusBadRequest)
		return
	}

	schedule, err := data.GetScheduleById(faculty.Schedule)

	if err != nil {
		logging.ErrorLogger.Println(err)
		w.WriteHeader(http.StatusBadRequest)
		return
	}

	scheduleDTOList := make([]ScheduleDTO, 0)
	for _, el := range schedule.Data {
		staff, err := data.GetStaffById(el.Staff)
		if err != nil {
			logging.ErrorLogger.Println(err)
			w.WriteHeader(http.StatusBadRequest)
			return
		}
		if staff.ID == staffLoggedIn.ID {
			for day, shifts := range el.Shifts {
				for _, shift := range shifts {
					scheduleDTOList = append(scheduleDTOList, ScheduleDTO{staff.Name + " " + staff.Surname + ", " + staff.Email, shift.Lecture, string(day), shift.Start + " - " + shift.End})
				}
			}
		}
	}

	encoder := json.NewEncoder(w)

	err = encoder.Encode(scheduleDTOList)
	if err != nil {
		logging.ErrorLogger.Println(err)
		w.WriteHeader(http.StatusBadRequest)
		return
	}

	w.WriteHeader(http.StatusOK)
	return
}

type ScheduleDTO struct {
	Staff string `json:"staff"`
	Lecture string `json:"lecture"`
	Day   string `json:"day"`
	Shift string `json:"shift"`
}

type IdDTO struct {
	ID string `json:"id"`
}
