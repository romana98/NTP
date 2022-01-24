use genevo::{operator::prelude::*, prelude::*, random::Rng, types::fmt::Display};
use crate::{models::algorithm::*, algorithm::preparation::prepare_data, enums::days::Day};
use std::collections::hash_map::Entry;
use std::collections::HashMap;
use rand::{self};

const POPULATION_SIZE: usize = 20;
const GENERATION_LIMIT: u64 = 15000;
const SELECTION_RATIO: f64 = 1.0;
const MUTATION_RATE: f64 = 1.0;
const REINSERTION_RATIO: f64 = 1.0;
const NUM_INDIVIDUALS_PER_PARENTS: usize = 3;
const MUTATION_MIN_VAL: i8 = 0;
const MUTATION_MAX_VAL: i8 = 4;
const NUM_OF_DAYS: i8 = 5;
static mut NUM_OF_CHROMOSOMES: i32 = 0;
static mut SIMULATOR: Option<Simulator> = None;
static mut SCHEDULE: Option<Schedule> = None;


/// How do the genes of the genotype show up in the phenotype
trait AsPhenotype {
    fn as_schedule(&self) -> Schedule;
}

impl AsPhenotype for Chromosomes {
    fn as_schedule(&self) -> Schedule {
        unsafe{
            match SCHEDULE {
                Some(ref sc) => {
                    let mut schedule_copy = Vec::new();

                    for assigned_shift in sc {
                        let mut lectures_copy = Vec::new();
                        for lecture in &assigned_shift.lectures {
                            lectures_copy.push(Lecture{name:(*lecture.name).to_owned(), number_of_times: lecture.number_of_times});
                        }

                        let mut shifts_copy: ShiftMap = HashMap::new();

                        for (k, v) in &assigned_shift.shifts {
                            let mut v_copy: Vec<Vertex> = Vec::new();

                            for vertex in v {
                                v_copy.push(Vertex{start: (*vertex.start).to_owned(), end: (*vertex.end).to_owned()});
                            }
                            shifts_copy.insert((*k).to_owned(), v_copy);
                        }
                        schedule_copy.push(AssignedShifts{
                            staff_id: assigned_shift.staff_id,
                            staff: (*assigned_shift.staff).to_owned(),
                            lectures: lectures_copy,
                            shifts: shifts_copy
                        });
                    }
                    return schedule_copy;
                },
                _ => {return Vec::new();}
            };
        };
    }
}

fn make_schedule(chromosomes: &Chromosomes) -> Schedule {
    let mut schedule: Schedule = Vec::new();
    unsafe{
        match SIMULATOR {
            Some(ref sim) =>{
                let mut index: i32 = -1;

                for staff in  &sim.staff {
                    let mut assigned_shifts: ShiftMap = HashMap::new();

                    for _ in 0..staff.num_of_shifts {
                        index += 1;
                        let chromosome = chromosomes.get(index as usize).unwrap();

                        let (day, shift) = get_day_and_shift(&get_available_shifts(&sim.shift_map, &assigned_shifts), chromosome.day_index);

                        match assigned_shifts.entry(day) {
                            Entry::Vacant(e) => { e.insert(vec![Vertex{start: shift.start.to_owned(), end: shift.end.to_owned()}]); },
                            Entry::Occupied(mut e) => { e.get_mut().push(Vertex{start: shift.start.to_owned(), end: shift.end.to_owned()}); }
                        }
                    }

                    let mut lectures_copy = Vec::new();
                    for lecture in &staff.lectures {
                        lectures_copy.push(Lecture{name: (*lecture.name).to_owned(), number_of_times: lecture.number_of_times});
                    }

                    schedule.push(AssignedShifts{staff_id: staff.id, staff: (*staff.full_name).to_owned(), lectures: lectures_copy, shifts: assigned_shifts })
                }
            },
            _ => {return Vec::new()}
        };
    };
    return schedule
}

fn get_day_and_shift(shift_map: &ShiftMap, mut day_idx: i8) -> (Day, Vertex) {
    let mut rng = rand::thread_rng();

    let days: Vec<Day> = vec![Day::Monday, Day::Tuesday, Day::Wednesday, Day::Thursday, Day::Friday];

    loop {
        let day = days.get(day_idx as usize);
        if day.is_some(){    
            let vertex = shift_map.get(day.unwrap());
            if vertex.is_some(){
                break;
            }else{
                day_idx = rng.gen_range(0..NUM_OF_DAYS);
            }
        }
    }
    let d = days.get(day_idx as usize).unwrap();
    let vertex = shift_map.get(d).unwrap();
    let mut num = vertex.len();
    if num != 1 {
        num -= 1;
    }

    let shift_idx = rng.gen_range(0..num);
    let shift = vertex.get(shift_idx).unwrap();
    let shift_copy = Vertex{start: (*shift.start).to_owned(), end: (*shift.end).to_owned()};
    let day = days.get(day_idx as usize).unwrap().to_owned();

    (day, shift_copy)
}

fn get_available_shifts(shift_map: &ShiftMap, assigned_shifts: &ShiftMap) -> ShiftMap {

    let mut available: ShiftMap = HashMap::new();

    for (k_sm, v_sm) in shift_map {
        if !assigned_shifts.contains_key(&k_sm){
            let mut v_sm_copy: Vec<Vertex> = Vec::new();
            for e in v_sm {
                v_sm_copy.push(Vertex{start: (*e.start).to_owned(), end: (*e.end).to_owned()});
            }
            available.insert((*k_sm).to_owned(), v_sm_copy);

        }else {
            let v_as = &assigned_shifts[&k_sm];
            for e_v_sm in v_sm {
                let vertex_sm = e_v_sm;
                let mut is_in = 0;

                for e_v_as in v_as{
                    if e_v_as.start == vertex_sm.start && e_v_as.end == vertex_sm.end{
                        is_in = 1;
                    }
                }

                if is_in == 0 {
                    match available.entry((*k_sm).to_owned()) {
                        Entry::Vacant(e) => {e.insert(vec![Vertex{start: vertex_sm.start.to_owned(), end: vertex_sm.end.to_owned()}]);},
                        Entry::Occupied(mut e) => { 
                            e.get_mut().push(Vertex{start: (*vertex_sm.start).to_owned(), end: (*vertex_sm.end).to_owned()}); }
                    }  
                }
            }
        }   
    }
    available
}

fn end_criteria(schedule: &Schedule) -> bool{
    unsafe{
        match SIMULATOR {
            Some(ref sim) => {
                let hc = &sim.end_criteria;
               
                for assigned_shift in schedule {
                    let mut weekly = 0;

                    for ( _, v) in &assigned_shift.shifts {
                        weekly += v.len();
                        if v.len() > hc.daily_max as usize {
                            return false;
                        }
                    }
                    if weekly > hc.weekly_max as usize || weekly < hc.weekly_min as usize{
                        return false;
                    }
                }

                for (k_sm, v_sm) in &sim.shift_map{
                    
                    for vertex in v_sm {
                        let mut num_of_times = 0;

                        for assigned_shift in schedule {
                            for ( k_as, v_as) in &assigned_shift.shifts {
                                for el in v_as {
                                    if k_sm == k_as && el.start == vertex.start && el.end == vertex.end {
                                        num_of_times += 1;
                                    }
                                }
                            }
                            if num_of_times > hc.max_per_shift {
                                return false;
                            }
                        }
                    } 
                }
            },
                _ => {return false}
            };
        };
    true
}

/// The fitness function for `Chromosomes`.
#[derive(Clone, Debug)]
struct FitnessCalc;

impl FitnessFunction<Chromosomes, usize> for FitnessCalc {
    fn fitness_of(&self, chromosomes: &Chromosomes) -> usize {
        let schedule = make_schedule(chromosomes);
        let mut fitness = 0;
        unsafe{
            match SIMULATOR {
                Some(ref sim) => {
                    for assigned_shift in &schedule {
                        let mut sum = 0;
                        let sc = sim.soft_constraints_by_staff.get(&assigned_shift.staff_id).unwrap();
                        for (k, v) in &sc.prefers {
                            for (day, vec) in &assigned_shift.shifts {
                                if k == &day.to_string() {
                                    sum += vec.len() as i32 * v;
                                }
                            }
                        }
                        fitness += sum;
                    }
                },
                    _ => return 0
            };
        };
       
        if end_criteria(&schedule){
            fitness = 1000;
        }
        unsafe{
            SCHEDULE = Some(schedule);
        }
        fitness as usize
    }

    fn average(&self, values: &[usize]) -> usize {
        (values.iter().sum::<usize>() as f32 / values.len() as f32 + 0.5).floor() as usize
    }

    fn highest_possible_fitness(&self) -> usize {
        1000
    }

    fn lowest_possible_fitness(&self) -> usize {
        0
    }
}

impl RandomValueMutation for Chromosome {
    fn random_mutated<R>(_value: Self, min_value: &Chromosome, max_value: &Chromosome, rng: &mut R) -> Self
    where
        R: Rng + Sized,
    {
        Chromosome {
            day_index: rng.gen_range(min_value.day_index..max_value.day_index)
        }
    }
}

/// Generate some random boards
struct Rotation;

impl GenomeBuilder<Chromosomes> for Rotation {
    fn build_genome<R>(&self, _: usize, rng: &mut R) -> Chromosomes
    where
        R: Rng + Sized,
    {    
        unsafe {
            (0..NUM_OF_CHROMOSOMES)
            .map(|_| Chromosome {
                day_index: rng.gen_range(0..MUTATION_MAX_VAL),
            })
            .collect()
        }
    }
}

pub fn start(faculty: &FacultySchedule, staff: &Vec<StaffSchedule>, lectures: &Vec<LectureSchedule>) -> Schedule {
    process_data(faculty, staff, lectures);

    let initial_population: Population<Chromosomes> = build_population()
        .with_genome_builder(Rotation)
        .of_size(POPULATION_SIZE)
        .uniform_at_random();

    let mut simulation = simulate(
        genetic_algorithm()
            .with_evaluation(FitnessCalc)
            .with_selection(RouletteWheelSelector::new(SELECTION_RATIO, NUM_INDIVIDUALS_PER_PARENTS))
            .with_crossover(MultiPointCrossBreeder::new(2))
            .with_mutation(RandomValueMutator::new(
                MUTATION_RATE,
                Chromosome { day_index: MUTATION_MIN_VAL },
                Chromosome { day_index: MUTATION_MAX_VAL }
            ))
            .with_reinsertion(ElitistReinserter::new(
                FitnessCalc,
                false,
                REINSERTION_RATIO,
            ))
            .with_initial_population(initial_population)
            .build(),
    )
    .until(or(
        FitnessLimit::new(FitnessCalc.highest_possible_fitness()),
        GenerationLimit::new(GENERATION_LIMIT),
    ))
    .build();

    let mut schedule: Schedule = Vec::new();
    loop {
        let result = simulation.step(); 
        match result {
            Ok(SimResult::Intermediate(step)) => {
                let evaluated_population = step.result.evaluated_population;
                let best_solution = step.result.best_solution;
                info!(
                    "Step: generation: {}, average_fitness: {}, \
                     best fitness: {}, duration: {}, processing_time: {}",
                    step.iteration,
                    evaluated_population.average_fitness(),
                    best_solution.solution.fitness,
                    step.duration.fmt(),
                    step.processing_time.fmt()
                );
            }
            Ok(SimResult::Final(step, processing_time, duration, stop_reason)) => {
                let best_solution = step.result.best_solution;
                info!("{}", stop_reason);
                info!(
                    "Final result after {}: generation: {}, \
                     best solution with fitness {} found in generation {}, processing_time: {}",
                    duration.fmt(),
                    step.iteration,
                    best_solution.solution.fitness,
                    best_solution.generation,
                    processing_time.fmt()
                );
                schedule = best_solution.solution.genome.as_schedule();
                break;
            }
            Err(error) => {
                info!("{}", error);
                break;
            }
        } 
    }
    return schedule;
}

fn process_data(faculty: &FacultySchedule, staff: &Vec<StaffSchedule>, lectures: &Vec<LectureSchedule>){
    unsafe {
        let (sim, num) =  prepare_data(faculty, staff, lectures);
        SIMULATOR = sim;
        NUM_OF_CHROMOSOMES = num;
    };
}