package algorithm

import (
	"github.com/romana98/NTP/data"
	ga "github.com/tomcraven/goga"
	"go.mongodb.org/mongo-driver/bson/primitive"
	"math/rand"
)

func (bc *BitsetCreate) Go() ga.Bitset {
	bitset := ga.Bitset{}
	bitset.Create(bc.size)
	for i := 0; i < bc.size; i++ {
		bitset.Set(i, rand.Intn(numberOfDays))
	}
	return bitset
}

type EliteConsumer struct {
	currentIter     int
	previousFitness int
}

type MaterSimulator struct {
	endCriteria            data.HardConstraint
	shiftMap               data.ShiftMap
	staff                  []RotationStaff
	softConstraintsByStaff map[primitive.ObjectID]data.SoftConstraint
	eliteSchedule          []data.AssignedShifts
	iteration              int
}

func (ms *MaterSimulator) OnBeginSimulation() {
}

func (ms *MaterSimulator) Simulate(genome *ga.IGenome) {
	rotation := newRotation(*genome, ms.staff, ms.shiftMap)
	fitness := rotation.GetFitness(ms.softConstraintsByStaff)
	(*genome).SetFitness(fitness)
	ms.eliteSchedule = rotation.schedule
	ms.iteration++
}

func (ms *MaterSimulator) OnEndSimulation() {
}

func (ms *MaterSimulator) ExitFunc(genome *ga.IGenome) bool {
	rotation := newRotation(*genome, ms.staff, ms.shiftMap)
	if ms.iteration > maxIteration {
		return true
	}

	for i := 0; i < len(rotation.schedule); i++ {
		assignedShifts := rotation.schedule[i]

		weekly := 0
		for _, v := range assignedShifts.Shifts {
			weekly += len(v)
			if len(v) > ms.endCriteria.DailyMax {
				return false
			}
		}
		if weekly > ms.endCriteria.WeeklyMax || weekly < ms.endCriteria.WeeklyMin {
			return false
		}
	}
	for kSM, vSM := range ms.shiftMap {
		keyMS := kSM
		for i := 0; i < len(vSM); i++ {
			valMS := vSM[i]
			numOfTimes := 0

			for _, el := range rotation.schedule {
				assignedShifts := el

				for kAS, vAS := range assignedShifts.Shifts {
					for k := 0; k < len(vAS); k++ {
						if keyMS == kAS && vAS[k].Start == valMS.Start && vAS[k].End == valMS.End {
							numOfTimes++
						}
					}
				}
				if numOfTimes > ms.endCriteria.MaxPerShift {
					return false
				}
			}
		}
	}

	return true
}

type BitsetCreate struct {
	size int
}

func (ec *EliteConsumer) OnElite(g *ga.IGenome) {
	fitness := (*g).GetFitness()
	/*if ec.currentIter > 0 {

		fmt.Println(ec.currentIter, "\t", fitness, "\t", ec.previousFitness)
	}*/
	ec.currentIter++
	ec.previousFitness = fitness
}
