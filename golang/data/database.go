package data

import (
	"context"
	"github.com/romana98/NTP/enum"
	"go.mongodb.org/mongo-driver/bson/primitive"
	"go.mongodb.org/mongo-driver/mongo"
	"go.mongodb.org/mongo-driver/mongo/options"
	"go.mongodb.org/mongo-driver/mongo/readpref"
	"log"
	"os"
	"time"
)

var collectionAdmin,
	collectionStaff,
	collectionFaculty,
	collectionSchedule,
	collectionShift,
	collectionSoftConstraint,
	collectionHardConstraint,
	collectionLecture *mongo.Collection

const (
	//port                       = "mongodb://localhost:27018"
	name                       = "user_db"
	collectionDBAdmin          = "admin"
	collectionDBStaff          = "staff"
	collectionDBShift          = "shift"
	collectionDBSoftConstraint = "soft_constraint"
	collectionDBHardConstraint = "hard_constraint"
	collectionDBLecture        = "lecture"
	collectionDBFaculty        = "faculty"
	collectionDBSchedule       = "schedule"
)

func connectDB(uri string) (*mongo.Client, context.Context,
	context.CancelFunc, error) {

	// ctx will be used to set deadline for process, here
	// deadline will of 30 seconds.
	ctx, cancel := context.WithTimeout(context.Background(),
		30*time.Second)

	// mongo.Connect return mongo.Client method
	client, err := mongo.Connect(ctx, options.Client().ApplyURI(uri))
	return client, ctx, cancel, err
}

// This is a user defined method that accepts
// mongo.Client and context.Context
// This method used to ping the mongoDB, return error if any.
func pingDB(client *mongo.Client, ctx context.Context) bool {

	if err := client.Ping(ctx, readpref.Primary()); err != nil {
		log.New(os.Stdout, "ERROR: ", log.Ltime|log.Lshortfile).Println(err)
		return false
	}

	return true
}

func InitDatabase() {

	client, ctx, _, err := connectDB(os.Getenv("MONGODB_URI"))
	if err != nil {
		log.New(os.Stdout, "ERROR: ", log.Ltime|log.Lshortfile).Println(err)
		return
	}

	// Check the connection
	if pingDB(client, ctx) {
		// Set collections for further use
		err := client.Database(name).Drop(ctx)
		if err != nil {
			log.New(os.Stdout, "ERROR: ", log.Ltime|log.Lshortfile).Println(err)
			return
		}
		setCollections(client)
		fillDB()
	}
	log.New(os.Stdout, "INFO: ", log.Ltime|log.Lshortfile).Println("Connected to MongoDB")
}

func setCollections(client *mongo.Client) {
	collectionShift = client.Database(name).Collection(collectionDBShift)
	collectionSoftConstraint = client.Database(name).Collection(collectionDBSoftConstraint)
	collectionHardConstraint = client.Database(name).Collection(collectionDBHardConstraint)
	collectionLecture = client.Database(name).Collection(collectionDBLecture)
	collectionStaff = client.Database(name).Collection(collectionDBStaff)
	collectionFaculty = client.Database(name).Collection(collectionDBFaculty)
	collectionSchedule = client.Database(name).Collection(collectionDBSchedule)
	collectionAdmin = client.Database(name).Collection(collectionDBAdmin)
}

func fillDB() {

	faculty := new(Faculty)
	res, err := SaveFaculty(faculty)
	faculty.ID = res.InsertedID.(primitive.ObjectID)

	fillAdmin()
	fillShifts(faculty)
	fillHardConstraints(faculty)

	lectureNames := [28]string{
		"oop1", "ntp", "web", "kts", "sbnz", "soft", "ml", "nvt", "xml", "alg",
		"bp", "npea", "lprs", "ori", "os", "mreze", "spp", "pp", "op", "oop2", "bsep",
		"isa", "mrs", "pigkut", "usi", "soc teh", "eng", "hci"}

	staffNames := [4]string{"John", "Mary", "Kevin", "Lucy"}
	staffSurnames := [4]string{"Doe", "Poppins", "Heck", "Gray"}
	staffEmails := [4]string{"j.doe@prof.com", "m.poppins@prof.com", "k.heck@prof.com", "l.gray@prof.com"}

	sc := []map[enum.DAY]int{
		{enum.Monday: 1, enum.Tuesday: 2, enum.Wednesday: 3, enum.Thursday: 4, enum.Friday: 5},
		{enum.Monday: 4, enum.Tuesday: 2, enum.Wednesday: 2, enum.Thursday: 1, enum.Friday: 2},
		{enum.Monday: 3, enum.Tuesday: 2, enum.Wednesday: 5, enum.Thursday: 4, enum.Friday: 5},
		{enum.Monday: 1, enum.Tuesday: 1, enum.Wednesday: 3, enum.Thursday: 4, enum.Friday: 1},
	}

	for i := 0; i < 4; i++ {

		scID := fillSoftConstraints(sc[i])
		lecturesID := make([]primitive.ObjectID, 0)
		fillLectures(&lecturesID, faculty, lectureNames[7*i:7*(i+1)])
		fillStaffs(lecturesID, scID, faculty, staffNames[i], staffSurnames[i], staffEmails[i])
	}

	faculty.Name = "ftn"
	_, err = UpdateFaculty(faculty)
	if err != nil {
		return
	}

}

func fillAdmin() {

	a := NewAdmin("admin", "admin", "a.admin@admin.com", "123qweasd")

	_, err := SaveAdmin(a)
	if err != nil {
		log.New(os.Stdout, "ERROR: ", log.Ltime|log.Lshortfile).Println(err)
	}

}

func fillShifts(faculty *Faculty) {
	starts := [8]string{"8:00", "10:45", "12:30", "14:15", "17:00", "19:45"}
	ends := [8]string{"10:30", "12:15", "14:00", "16:45", "19:30", "21:15"}
	days := [5]enum.DAY{enum.Monday, enum.Tuesday, enum.Wednesday, enum.Thursday, enum.Friday}

	shits := make([]primitive.ObjectID, 0)

	for i := 0; i < 6; i++ {
		for j := 0; j < 5; j++ {
			s := NewShift(starts[i], ends[i], days[j])
			res, err := SaveShift(s)
			if err != nil {
				log.New(os.Stdout, "ERROR: ", log.Ltime|log.Lshortfile).Println(err)
			}
			shits = append(shits, res.InsertedID.(primitive.ObjectID))
		}
	}

	faculty.Shifts = shits

}

func fillHardConstraints(faculty *Faculty) {

	hc := NewHardConstraint(4, 15, 5, 3)
	res, err := SaveHardConstraint(hc)
	if err != nil {
		log.New(os.Stdout, "ERROR: ", log.Ltime|log.Lshortfile).Println(err)
	}

	faculty.HardConstraint = res.InsertedID.(primitive.ObjectID)

}

func fillSoftConstraints(prefers map[enum.DAY]int) primitive.ObjectID {

	sc := NewSoftConstraint(prefers)
	res, err := SaveSoftConstraint(sc)
	if err != nil {
		log.New(os.Stdout, "ERROR: ", log.Ltime|log.Lshortfile).Println(err)
	}
	return res.InsertedID.(primitive.ObjectID)

}

func fillLectures(lecturesID *[]primitive.ObjectID, faculty *Faculty, names []string) {

	for i := 1; i < 5; i++ {
		lecture := NewLecture(i, names[i-1])
		res, err := SaveLecture(lecture)
		if err != nil {
			log.New(os.Stdout, "ERROR: ", log.Ltime|log.Lshortfile).Println(err)
			return
		}

		*lecturesID = append(*lecturesID, res.InsertedID.(primitive.ObjectID))
	}

	faculty.Lectures = append(faculty.Lectures, *lecturesID...)

}

func fillStaffs(lecturesID []primitive.ObjectID, scID primitive.ObjectID, faculty *Faculty, name, surname, email string) {

	staffListID := make([]primitive.ObjectID, 0)

	p := NewStaff(name, surname, email, lecturesID, scID, faculty.ID)

	res, err := SaveStaff(p)
	if err != nil {
		log.New(os.Stdout, "ERROR: ", log.Ltime|log.Lshortfile).Println(err)
	}
	staffListID = append(faculty.Staff, res.InsertedID.(primitive.ObjectID))

	faculty.Staff = staffListID

}
