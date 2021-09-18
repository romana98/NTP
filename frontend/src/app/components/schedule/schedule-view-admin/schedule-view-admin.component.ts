import {Component, OnInit, ViewChild} from '@angular/core';
import {MatSnackBar} from "@angular/material/snack-bar";
import {FormBuilder, FormGroup, FormGroupDirective, Validators} from "@angular/forms";
import {FacultyService} from "../../../services/faculty.service";
import {ScheduleService} from "../../../services/schedule.service";

@Component({
  selector: 'app-schedule-view-admin',
  templateUrl: './schedule-view-admin.component.html',
  styleUrls: ['./schedule-view-admin.component.css']
})
export class ScheduleViewAdminComponent implements OnInit {

  @ViewChild(FormGroupDirective) formGroupDirective: FormGroupDirective | undefined;

  schedule = []

  facultyList: { name: string, id: string, schedules: string }[] = []
  form: FormGroup;


  constructor(private scheduleService: ScheduleService,
              private fb: FormBuilder,
              private facultyService: FacultyService,
              public snackBar: MatSnackBar) {

    this.form = this.fb.group({
      faculty: [null, Validators.required],

    });
  }

  ngOnInit(): void {
    this.getFaculties();
  }

  getSchedule() {
    let faculty = this.form.controls['faculty'].value

    if (faculty.schedule === undefined || faculty.schedule === "000000000000000000000000") {
      this.snackBar.open("Faculty doesn't have a schedule", "Close", {duration: 2000});
    } else {
      this.scheduleService.getSchedule(faculty.schedule).toPromise().then((res) => {
        this.schedule = res
      }, (err) => {
        this.snackBar.open("Server error: " + err.status, "Close", {duration: 2000});
      })
    }
  }

  generateSchedule() {
    let faculty = this.form.controls['faculty'].value

    this.scheduleService.generateSchedule({id: faculty.id}).toPromise().then((res) => {
      this.schedule = res
      // @ts-ignore
      setTimeout(() => this.formGroupDirective.resetForm(), 0);
      this.getFaculties()
    }, (err) => {
      this.snackBar.open("Server error: " + err.status, "Close", {duration: 2000});
    })
  }

  getFaculties() {
    this.facultyService.getFaculties().toPromise().then((res) => {
      this.facultyList = res.map((item: { name: any; id: any; schedule: any }) => {
        return {name: item.name, id: item.id, schedule: item.schedule}
      });
    }, (err) => {
      this.snackBar.open("Server error: " + err.status, "Close", {duration: 2000});
    })
  }

}
