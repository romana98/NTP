package routing

import (
	"github.com/gorilla/mux"
	"github.com/romana98/NTP/handlers"
	"net/http"
)

func NewRouter() *mux.Router {

	r := mux.NewRouter()

	//Auth
	r.HandleFunc("/auth/log-in", handlers.LogIn).Methods(http.MethodPost)

	//Admin
	r.HandleFunc("/admins", handlers.NewAdminHandler).Methods(http.MethodPost)

	//Shift
	r.HandleFunc("/shifts", handlers.GetShifts).Methods(http.MethodGet)
	r.HandleFunc("/shifts", handlers.AddShift).Methods(http.MethodPost)
	r.HandleFunc("/shifts/{id}", handlers.GetShift).Methods(http.MethodGet)
	r.HandleFunc("/shifts", handlers.UpdateShift).Methods(http.MethodPut)
	r.HandleFunc("/shifts/{id}", handlers.DeleteShift).Methods(http.MethodDelete)

	//Lecture
	r.HandleFunc("/lectures", handlers.GetLectures).Methods(http.MethodGet)
	r.HandleFunc("/lectures/by-staff", handlers.GetLecturesByStaff).Methods(http.MethodGet)
	r.HandleFunc("/lectures", handlers.AddLecture).Methods(http.MethodPost)
	r.HandleFunc("/lectures/{id}", handlers.GetLecture).Methods(http.MethodGet)
	r.HandleFunc("/lectures", handlers.UpdateLecture).Methods(http.MethodPut)
	r.HandleFunc("/lectures/{id}", handlers.DeleteLecture).Methods(http.MethodDelete)
	r.HandleFunc("/lectures/by-faculty", handlers.GetLecturesByIds).Methods(http.MethodPost)

	//Hard Constraint
	r.HandleFunc("/hard-constraints", handlers.GetHardConstraints).Methods(http.MethodGet)
	r.HandleFunc("/hard-constraints", handlers.AddHardConstraint).Methods(http.MethodPost)
	r.HandleFunc("/hard-constraints/{id}", handlers.GetHardConstraint).Methods(http.MethodGet)
	r.HandleFunc("/hard-constraints", handlers.UpdateHardConstraint).Methods(http.MethodPut)
	r.HandleFunc("/hard-constraints/{id}", handlers.DeleteHardConstraint).Methods(http.MethodDelete)

	//Staff
	r.HandleFunc("/staff", handlers.GetStaffList).Methods(http.MethodGet)
	r.HandleFunc("/staff", handlers.AddStaff).Methods(http.MethodPost)
	r.HandleFunc("/staff/{id}", handlers.GetStaff).Methods(http.MethodGet)
	r.HandleFunc("/staff", handlers.UpdateStaff).Methods(http.MethodPut)
	r.HandleFunc("/staff/{id}", handlers.DeleteStaff).Methods(http.MethodDelete)

	//Soft Constraint
	r.HandleFunc("/soft-constraints", handlers.GetSoftConstraintsByStaff).Methods(http.MethodGet)
	r.HandleFunc("/soft-constraints", handlers.UpdateSoftConstraint).Methods(http.MethodPut)

	//Faculty
	r.HandleFunc("/faculties", handlers.GetFaculties).Methods(http.MethodGet)
	r.HandleFunc("/faculties", handlers.AddFaculty).Methods(http.MethodPost)
	r.HandleFunc("/faculties/{id}", handlers.GetFaculty).Methods(http.MethodGet)
	r.HandleFunc("/faculties", handlers.UpdateFaculty).Methods(http.MethodPut)
	r.HandleFunc("/faculties/{id}", handlers.DeleteFaculty).Methods(http.MethodDelete)

	//Schedule
	r.HandleFunc("/schedule", handlers.GenerateSchedule).Methods(http.MethodPost)
	r.HandleFunc("/schedule/by-staff", handlers.GetScheduleByStaff).Methods(http.MethodGet)
	r.HandleFunc("/schedule/{id}", handlers.GetSchedule).Methods(http.MethodGet)

	return r
}
