package handlers

import (
	"github.com/romana98/NTP/enum"
	"net/http"
)

func NewAdminHandler(w http.ResponseWriter, r *http.Request) {

	isAuth := checkToken(r, string(enum.Admin))
	if isAuth != http.StatusOK {
		w.WriteHeader(isAuth)
	}

}
