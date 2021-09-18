import {Component, OnInit, ViewChild} from '@angular/core';
import {FormBuilder, FormGroup, FormGroupDirective, Validators} from "@angular/forms";
import {MatSnackBar} from "@angular/material/snack-bar";
import {StaffService} from "../../../services/staff.service";
import {StaffModel} from "../../../model/staff-model";
import {FacultyService} from "../../../services/faculty.service";
import {LectureService} from "../../../services/lecture.service";

@Component({
  selector: 'app-staff-crud',
  templateUrl: './staff-crud.component.html',
  styleUrls: ['./staff-crud.component.css']
})
export class StaffCrudComponent implements OnInit {

  @ViewChild(FormGroupDirective) formGroupDirective: FormGroupDirective | undefined;

  staffList = []

  facultyList: { name: string, id: string, lectures: string[] }[] = []
  lectureList: { name: string, id: string }[] = []

  form: FormGroup;
  formEdit: FormGroup;
  isHidden = true;

  constructor(private fb: FormBuilder,
              private staffService: StaffService,
              private facultyService: FacultyService,
              private lectureService: LectureService,
              public snackBar: MatSnackBar) {

    this.form = this.fb.group({
      email: [null, [Validators.required, Validators.email]],
      name: [null, Validators.required],
      surname: [null, Validators.required],
      lectures: [[], Validators.required],
      faculty: [null, Validators.required],

    });
    this.formEdit = this.fb.group({
      id: [null, Validators.required],
      email: [null, [Validators.required, Validators.email]],
      name: [null, Validators.required],
      surname: [null, Validators.required],
      lectures: [[], Validators.required],
      faculty: [null, Validators.required],

    });
  }

  ngOnInit(): void {
    this.getStaffList();
    this.getFaculties();
  }

  addStaff() {
    this.staffService.createStaff(new StaffModel("",
      this.form.controls['email'].value, this.form.controls['name'].value,
      this.form.controls['surname'].value, this.form.controls['lectures'].value,
      this.form.controls['faculty'].value)).toPromise().then(() => {
      this.snackBar.open("Successfully created staff", "Close", {duration: 2000});
      this.getStaffList();
      // @ts-ignore
      setTimeout(() => this.formGroupDirective.resetForm(), 0);
    }, () => {
      this.snackBar.open("Staff already exists.", "Close", {duration: 2000});
    })
  }

  deleteStaff(id: number) {
    this.staffService.deleteStaff(id).toPromise().then(() => {
      this.snackBar.open("Successfully deleted staff", "Close", {duration: 2000});
      this.getStaffList();
    }, () => {
      this.snackBar.open("Staff is part of a faculty.", "Close", {duration: 2000});
    })
  }

  editStaff() {
    this.isHidden = true;
    this.staffService.updateStaff(new StaffModel(this.formEdit.controls['id'].value,
      this.formEdit.controls['email'].value, this.formEdit.controls['name'].value,
      this.formEdit.controls['surname'].value, this.formEdit.controls['lectures'].value,
      this.formEdit.controls['faculty'].value)).toPromise().then(() => {
      this.snackBar.open("Successfully updated staff", "Close", {duration: 2000});
      this.isHidden = true;
      this.getStaffList();
    }, () => {
      this.snackBar.open("Error updating staff", "Close", {duration: 2000});
    })
  }

  editModeOn(id: number) {
    this.staffService.getStaff(id).toPromise().then((resStaff) => {
      this.facultyService.getFaculty(resStaff.faculty).toPromise().then((res) => {
        this.getLectures(res.lectures)
        this.formEdit.patchValue({
          id: resStaff.id,
          email: resStaff.email,
          name: resStaff.name,
          surname: resStaff.surname,
          lectures: resStaff.lectures,
          faculty: resStaff.faculty,

        });
      }, () => {
        this.snackBar.open("Faculty with id doesn't exist.", "Close", {duration: 2000});
      })

      this.isHidden = false;
    }, () => {
      this.snackBar.open("Staff with id doesn't exists.", "Close", {duration: 2000});
    })
  }

  getStaffList() {
    this.staffService.getStaffList().toPromise().then((res) => {

      this.staffList = res
    }, (err) => {
      this.snackBar.open("Server error: " + err.status, "Close", {duration: 2000});
    })
  }

  getFaculties() {
    this.facultyService.getFaculties().toPromise().then((res) => {
      this.facultyList = res.map((item: { name: any; id: any; lectures: any }) => {
        return {name: item.name, id: item.id, lectures: item.lectures}
      });
    }, (err) => {
      this.snackBar.open("Server error: " + err.status, "Close", {duration: 2000});
    })
  }

  getLectures(lectures: string[]) {
    this.lectureService.getLecturesByFaculty(lectures).toPromise().then((res) => {
      this.lectureList = res
    }, (err) => {
      this.snackBar.open("Server error: " + err.status, "Close", {duration: 2000});
    })
  }

}
