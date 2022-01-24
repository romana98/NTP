package algorithm

import (
	"github.com/romana98/NTP/data"
	"github.com/romana98/NTP/enum"
	"github.com/romana98/NTP/logging"
	ga "github.com/tomcraven/goga"
	"go.mongodb.org/mongo-driver/bson/primitive"
	"math/rand"
	"runtime"
	"time"
)

const (
	populationSize = 20
	numberOfDays   = 5
	numThreads     = 10
	maxIteration   = 15000
)

type Rotation struct {
	schedule []data.AssignedShifts
}

type RotationStaff struct {
	staff       data.Staff
	lectures    []data.Lecture
	numOfShifts int
}

func newRotation(genome ga.IGenome, staff []RotationStaff, shiftMap data.ShiftMap) *Rotation {
	r := &Rotation{
		schedule: make([]data.AssignedShifts, 0),
	}

	// for each employee go over the bitset and extract the current shift assigment for the whole period of time
	bits := genome.GetBits()
	var index int
	for i := 0; i < len(staff); i++ {
		assignedShifts := data.AssignedShifts{
			Staff:    staff[i].staff,
			Lectures: staff[i].lectures,
			Shifts:   make(data.ShiftMap),
		}

		for j := 0; j < staff[i].numOfShifts; j, index = j+1, index+1 {
			chromosome := bits.Get(index)

			day, shift := getDayAndShift(getAvailableShifts(shiftMap, assignedShifts.Shifts), Abs(chromosome))
			assignedShifts.Shifts[day] = append(assignedShifts.Shifts[day], shift)
		}
		r.schedule = append(r.schedule, assignedShifts)
	}

	return r
}

func getDayAndShift(shiftMap data.ShiftMap, dayIdx int) (enum.DAY, data.Vertex) {
	days := [5]enum.DAY{enum.Monday, enum.Tuesday, enum.Wednesday, enum.Thursday, enum.Friday}

	for len(shiftMap[days[dayIdx]]) == 0 {
		dayIdx = Abs(rand.Intn(numberOfDays))
	}

	var num int
	if num = len(shiftMap[days[dayIdx]]); len(shiftMap[days[dayIdx]]) != 1 {
		num = len(shiftMap[days[dayIdx]]) - 1
	}
	shiftIdx := rand.Intn(num)
	shift := shiftMap[days[dayIdx]][shiftIdx]
	return days[dayIdx], shift
}

func getAvailableShifts(shiftMap, assignedShifts data.ShiftMap) data.ShiftMap {

	available := make(data.ShiftMap)

	for kSM, vSM := range shiftMap {
		if _, ok := assignedShifts[kSM]; !ok {
			available[kSM] = vSM
		} else {
			vAS := assignedShifts[kSM]
			for i := 0; i < len(vSM); i++ {
				vertexSM := vSM[i]
				isIn := 0
				for j := 0; j < len(vAS); j++ {
					if vAS[j].Start == vertexSM.Start && vAS[j].End == vertexSM.End {
						isIn = 1
					}
				}
				if isIn == 0 {
					available[kSM] = append(available[kSM], vertexSM)
				}
			}
		}
	}

	return available
}

func (r Rotation) GetFitness(softConstraintsByStaff map[primitive.ObjectID]data.SoftConstraint) int {

	fitness := 0
	for _, el := range r.schedule {
		sum := 0
		for k, v := range softConstraintsByStaff[el.Staff.ID].Prefers {
			for day, arr := range el.Shifts {
				if k == day {
					sum += len(arr) * v
				}
			}
		}
		fitness += sum
	}
	return fitness
}

func Start(faculty *data.Faculty) {
	materSimulator := new(MaterSimulator)
	materSimulator.shiftMap = make(data.ShiftMap)

	shifts, err := data.GetAllShiftsByFaculty(faculty.Shifts)

	if err != nil {
		logging.ErrorLogger.Println(err)
		return
	}

	for _, element := range shifts {
		materSimulator.shiftMap[element.Day] = append(materSimulator.shiftMap[element.Day], data.Vertex{element.Start, element.End})
	}

	hc, err := data.GetHardConstraintById(faculty.HardConstraint)
	if err != nil {
		logging.ErrorLogger.Println(err)
		return
	}
	materSimulator.endCriteria = hc
	materSimulator.staff = make([]RotationStaff, 0)

	staffList, err := data.GetAllStaffByFaculty(faculty.Staff)
	if err != nil {
		logging.ErrorLogger.Println(err)
		return
	}
	var lectureLen int
	scByStaff := make(map[primitive.ObjectID]data.SoftConstraint)

	for i := 0; i < len(staffList); i++ {

		sc, _ := data.GetSoftConstraintById(staffList[i].SoftConstraints)
		scByStaff[staffList[i].ID] = sc

		lectures, _ := data.GetAllLecturesByIds(staffList[i].Lectures)

		lecturesPerStaff := 0
		for _, el := range lectures {
			lecturesPerStaff += el.NumberOfTimes
		}
		materSimulator.staff = append(materSimulator.staff, RotationStaff{
			staff:       staffList[i],
			lectures:    lectures,
			numOfShifts: lecturesPerStaff,
		})
		lectureLen += lecturesPerStaff

	}
	materSimulator.softConstraintsByStaff = scByStaff

	schedule := generateScheduler(len(staffList)*lectureLen, materSimulator)

	assignedShiftsByStaffId := make([]data.AssignedShiftsByStaffId, 0)

	for _, el := range schedule {
		lectures := make([]string, 0)
		for _, l := range el.Lectures {
			for i := 0; i < l.NumberOfTimes; i++ {
				lectures = append(lectures, l.Name)
			}
		}
		shiftLectureMap := make(data.ShiftLectureMap, 0)

		cnt := 0
		for k, v := range el.Shifts {
			shiftLectureMap[k] = make([]data.VertexLecture, 0)
			for _, vertex := range v {
				shiftLectureMap[k] = append(shiftLectureMap[k], data.VertexLecture{Start: vertex.Start, End: vertex.End, Lecture: lectures[cnt]})
				cnt++
			}
		}
		assignedShiftsByStaffId = append(assignedShiftsByStaffId, data.AssignedShiftsByStaffId{Staff: el.Staff.ID, Shifts: shiftLectureMap})
	}

	result, err := data.SaveSchedule(data.NewSchedule(assignedShiftsByStaffId))
	if err != nil {
		logging.ErrorLogger.Println(err)
		return
	}

	faculty.Schedule = result.InsertedID.(primitive.ObjectID)
	_, err = data.UpdateFaculty(faculty)

	if err != nil {
		logging.ErrorLogger.Println(err)
		return
	}

}

func generateScheduler(numOfStaffLecture int, materSimulator *MaterSimulator) []data.AssignedShifts {
	runtime.GOMAXPROCS(numThreads)

	// Mater - combining evolved genomes
	mater := ga.NewMater(
		[]ga.MaterFunctionProbability{
			{P: 1.0, F: ga.TwoPointCrossover},
			{P: 1.0, F: ga.Mutate},
			{P: 1.0, F: ga.UniformCrossover, UseElite: true},
		},
	)

	// selector defines how to select genomes from which the elite is being taken
	selector := ga.NewSelector(
		[]ga.SelectorFunctionProbability{
			// Roulette is a selection function that selects a genome where genomes that have a higher fitness are more likely to be picked
			{P: 1.0, F: ga.Roulette},
		},
	)

	algorithm := ga.NewGeneticAlgorithm()
	//Simulator - a simulation component used to score each genome in each generation
	algorithm.Simulator = materSimulator

	//EliteConsumer - an optional class that accepts the 'elite' of each population generation
	algorithm.EliteConsumer = &EliteConsumer{currentIter: 0, previousFitness: 0}

	//BitsetCreate - used to create the initial population of genomes
	algorithm.BitsetCreate = &BitsetCreate{size: numOfStaffLecture}

	//Selector - selection methods for next generation
	algorithm.Selector = selector

	// Mater - combining evolved genomes
	algorithm.Mater = mater

	algorithm.Init(populationSize, numThreads)

	startTime := time.Now()

	algorithm.Simulate()

	logging.InfoLogger.Println("Algorithm time: ", time.Since(startTime))

	return materSimulator.eliteSchedule
}

func Abs(x int) int {
	if x < 0 {
		return -x
	}
	return x
}
