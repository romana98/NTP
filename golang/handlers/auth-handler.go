package handlers

//https://www.sohamkamani.com/golang/jwt-authentication/#implementation-in-go

import (
	"encoding/json"
	"github.com/dgrijalva/jwt-go"
	"github.com/romana98/NTP/data"
	"github.com/romana98/NTP/enum"
	"net/http"
	"time"
)

// Create the JWT key used to create the signature
var jwtKey = []byte("my_secret_key")

// Credentials : Create a struct to read the username and password from the request body
type Credentials struct {
	Email    string `json:"email"`
	Password string `json:"password"`
}

// Token : Create a struct to send token to client
type Token struct {
	AccessToken string `json:"accessToken"`
}

// Claims : a struct that will be encoded to a JWT.
// We add jwt.StandardClaims as an embedded type, to provide fields like expiry time
type Claims struct {
	Email string `json:"email"`
	Role  string `json:"role"`
	jwt.StandardClaims
}

// LogIn : Create the log-in handler
func LogIn(w http.ResponseWriter, r *http.Request) {
	var credentials Credentials

	// Get the JSON body and decode into credentials
	err := json.NewDecoder(r.Body).Decode(&credentials)
	if err != nil {
		// If the structure of the body is wrong, return an HTTP error
		w.WriteHeader(http.StatusBadRequest)
		return
	}

	var role enum.ROLE
	var admin, okA = data.GetAdminByEmail(credentials.Email)
	var professor, okP = data.GetStaffByEmail(credentials.Email)
	// If a password exists for the given user
	// AND, if it is the same as the password we received, then we can move ahead
	// if NOT, then we return an "Unauthorized" status
	if okA != nil && okP != nil {
		w.WriteHeader(http.StatusUnauthorized)
		return
	}

	if okA == nil {
		if admin.Password == credentials.Password {
			role = admin.Role
		} else {
			w.WriteHeader(http.StatusUnauthorized)
			return
		}

	}

	if okP == nil {
		if professor.Password == credentials.Password {
			role = professor.Role
		} else {
			w.WriteHeader(http.StatusUnauthorized)
			return
		}
	}

	// Declare the expiration time of the token
	expirationTime := time.Now().Add(5 * time.Hour)

	// Create the JWT claims, which includes the username and expiry time
	claims := &Claims{
		Email: credentials.Email,
		Role:  string(role),
		StandardClaims: jwt.StandardClaims{
			// In JWT, the expiry time is expressed as unix milliseconds
			ExpiresAt: expirationTime.Unix(),
		},
	}

	// Declare the token with the algorithm used for signing, and the claims
	token := jwt.NewWithClaims(jwt.SigningMethodHS256, claims)
	// Create the JWT string
	tokenString, err := token.SignedString(jwtKey)
	if err != nil {
		// If there is an error in creating the JWT return an internal server error
		w.WriteHeader(http.StatusInternalServerError)
		return
	}

	// Finally, we set the client cookie for "token" as the JWT we just generated
	// we also set an expiry time which is the same as the token itself
	http.SetCookie(w, &http.Cookie{
		Name:    "accessToken",
		Value:   tokenString,
		Expires: expirationTime,
	})

	t := new(Token)
	t.AccessToken = tokenString

	tokenBytes, err := json.Marshal(t)

	if err != nil {
		w.WriteHeader(http.StatusInternalServerError)
		return
	}

	_, err = w.Write(tokenBytes)
	if err != nil {
		w.WriteHeader(http.StatusInternalServerError)
		return
	}

	w.WriteHeader(http.StatusOK)
}

// checkToken : checks if token is valid
func checkToken(r *http.Request, acceptedRole string) int {

	tokenString := r.Header.Get("Authorization")
	tokenString = tokenString[7:] //remove Bearer

	// Initialize a new instance of `Claims`
	claims := &Claims{}

	// Parse the JWT string and store the result in `claims`.
	// Note that we are passing the key in this method as well. This method will return an error
	// if the token is invalid (if it has expired according to the expiry time we set on sign in),
	// or if the signature does not match
	tkn, err := jwt.ParseWithClaims(tokenString, claims, func(token *jwt.Token) (interface{}, error) {
		return jwtKey, nil
	})

	if err != nil {
		if err == jwt.ErrSignatureInvalid {
			return http.StatusUnauthorized
		}
		return http.StatusBadRequest
	}

	if acceptedRole != string(enum.Both) && (claims.Role == string(enum.Admin) || claims.Role == string(enum.Staff)) {
		if claims.Role != acceptedRole {
			return http.StatusUnauthorized
		}
	}

	if !tkn.Valid {
		return http.StatusUnauthorized
	}

	return http.StatusOK
}

func getLoggedInEmail(token string) string {
	claims := &Claims{}

	_, err := jwt.ParseWithClaims(token, claims, func(token *jwt.Token) (interface{}, error) {
		return jwtKey, nil
	})

	if err != nil {
		return ""
	}

	return claims.Email
}
