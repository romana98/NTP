import {Component, OnInit, ViewChild} from '@angular/core';
import {FormBuilder, FormGroup, FormGroupDirective, Validators} from "@angular/forms";
import {FacultyService} from "../../../services/faculty.service";
import {MatSnackBar} from "@angular/material/snack-bar";
import {HardConstraintService} from "../../../services/hard-constraint.service";
import {HardConstraintModel} from "../../../model/hard-constraint-model";
import {ShiftService} from "../../../services/shift.service";
import {ShiftModel} from "../../../model/shift-model";
import {LectureModel} from "../../../model/lecture-model";
import {LectureService} from "../../../services/lecture.service";
import {FacultyModel} from "../../../model/faculty-model";
import {StaffModel} from "../../../model/staff-model";
import {StaffService} from "../../../services/staff.service";


@Component({
  selector: 'app-faculty-crud',
  templateUrl: './faculty-crud.component.html',
  styleUrls: ['./faculty-crud.component.css']
})
export class FacultyCrudComponent implements OnInit {

  @ViewChild(FormGroupDirective) formGroupDirective: FormGroupDirective | undefined;

  faculties = []
  hardConstraints: HardConstraintModel[] = []
  shifts: ShiftModel[] = []
  lectures: LectureModel[] = []
  staffList: StaffModel[] = []


  form: FormGroup;
  formEdit: FormGroup;
  isHidden = true;

  constructor(private fb: FormBuilder,
              private facultyService: FacultyService,
              private hardConstraintService: HardConstraintService,
              private shiftService: ShiftService,
              private lectureService: LectureService,
              private staffService: StaffService,
              public snackBar: MatSnackBar) {

    this.form = this.fb.group({
      name: [null, Validators.required],
      hardConstraint: [null, Validators.required],
      shifts: [null, Validators.required],
      lectures: [null, Validators.required],


    });
    this.formEdit = this.fb.group({
      id: [null, Validators.required],
      name: [null, Validators.required],
      hardConstraint: [null, Validators.required],
      shifts: [null, Validators.required],
      lectures: [null, Validators.required],
      schedule: [null],
    });
  }

  ngOnInit(): void {
    this.getFaculties();
    this.getHardConstraints();
    this.getShifts();
    this.getLectures();
    this.getStaffList();
  }


  addFaculty() {
    this.facultyService.createFaculty(new FacultyModel("", this.form.controls['name'].value,
      this.form.controls['hardConstraint'].value, this.form.controls['shifts'].value, [],
      this.form.controls['lectures'].value, "")).toPromise().then(() => {
      this.snackBar.open("Successfully created faculty", "Close", {duration: 2000});
      this.getFaculties();
      // @ts-ignore
      setTimeout(() => this.formGroupDirective.resetForm(), 0);
    }, () => {
      this.snackBar.open("Faculty already exists.", "Close", {duration: 2000});
    })
  }

  deleteFaculty(id: number) {
    this.facultyService.deleteFaculty(id).toPromise().then(() => {
      this.snackBar.open("Successfully deleted faculty", "Close", {duration: 2000});
      this.getFaculties();
    }, () => {
      this.snackBar.open("Faculty is part of a faculty.", "Close", {duration: 2000});
    })
  }

  editFaculty() {
    this.isHidden = true;
    this.facultyService.updateFaculty(new FacultyModel(this.formEdit.controls['id'].value, this.formEdit.controls['name'].value,
      this.formEdit.controls['hardConstraint'].value, this.formEdit.controls['shifts'].value, [],
      this.formEdit.controls['lectures'].value, this.formEdit.controls['schedule'].value)).toPromise().then(() => {
      this.snackBar.open("Successfully updated faculty", "Close", {duration: 2000});
      this.isHidden = true;
      this.getFaculties();
    }, () => {
      this.snackBar.open("Error updating faculty", "Close", {duration: 2000});
    })
  }

  editModeOn(id: any) {
    this.facultyService.getFaculty(id).toPromise().then((res) => {
      this.formEdit.patchValue({
        id: res.id,
        name: res.name,
        hardConstraint: res.hard_constraint,
        shifts: res.shifts,
        lectures: res.lectures,
        staffList: res.staff,
        schedule: res.schedule
      });
      this.isHidden = false;
    }, () => {
      this.snackBar.open("Faculty with id doesn't exist.", "Close", {duration: 2000});
    })
  }

  getFaculties() {
    this.facultyService.getFaculties().toPromise().then((res) => {
      if (!res) {
        this.faculties = []
      } else {
        this.faculties = res
      }

    }, (err) => {
      this.snackBar.open("Server error: " + err.status, "Close", {duration: 2000});
    })
  }

  getHardConstraints() {
    this.hardConstraintService.getHardConstraints().toPromise().then((res) => {
      this.hardConstraints = res
    }, (err) => {
      this.snackBar.open("Server error: " + err.status, "Close", {duration: 2000});
    })
  }

  getShifts() {
    this.shiftService.getShifts().toPromise().then((res) => {
      this.shifts = res
    }, (err) => {
      this.snackBar.open("Server error: " + err.status, "Close", {duration: 2000});
    })
  }

  getLectures() {
    this.lectureService.getLectures().toPromise().then((res) => {
      this.lectures = res;
    }, (err) => {
      this.snackBar.open("Server error: " + err.status, "Close", {duration: 2000});
    })
  }

  getStaffList() {
    this.staffService.getStaffList().toPromise().then((res) => {

      this.staffList = res
    }, (err) => {
      this.snackBar.open("Server error: " + err.status, "Close", {duration: 2000});
    })
  }

}
