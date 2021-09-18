import {Component, OnInit, ViewChild} from '@angular/core';
import {FormBuilder, FormGroup, FormGroupDirective, Validators} from "@angular/forms";
import {MatSnackBar} from "@angular/material/snack-bar";
import {LectureModel} from "../../../model/lecture-model";
import {LectureService} from "../../../services/lecture.service";

@Component({
  selector: 'app-lecture-crud',
  templateUrl: './lecture-crud.component.html',
  styleUrls: ['./lecture-crud.component.css']
})
export class LectureCrudComponent implements OnInit {

  @ViewChild(FormGroupDirective) formGroupDirective: FormGroupDirective | undefined;


  lectures = []

  form: FormGroup;
  formEdit: FormGroup;
  isHidden = true;

  constructor(private fb: FormBuilder,
              private lectureService: LectureService,
              public snackBar: MatSnackBar) {

    this.form = this.fb.group({
      id: [null, Validators.required],
      name: [null, Validators.required],
      numberOfTimes: [null, Validators.required]

    });
    this.formEdit = this.fb.group({
      id: [null, Validators.required],
      name: [null, Validators.required],
      numberOfTimes: [null, Validators.required]

    });
  }

  ngOnInit(): void {
    this.getLectures();
  }

  addLecture() {
    this.lectureService.createLecture(new LectureModel("",
      this.form.controls['name'].value, this.form.controls['numberOfTimes'].value)).toPromise().then(() => {
      this.snackBar.open("Successfully created lecture", "Close", {duration: 2000});
      this.getLectures();
      // @ts-ignore
      setTimeout(() => this.formGroupDirective.resetForm(), 0);
    }, () => {
      this.snackBar.open("Lecture already exists.", "Close", {duration: 2000});
    })
  }

  deleteLecture(id: number) {
    this.lectureService.deleteLecture(id).toPromise().then(() => {
      this.snackBar.open("Successfully deleted lecture", "Close", {duration: 2000});
      this.getLectures();
    }, () => {
      this.snackBar.open("Lecture is part of a faculty.", "Close", {duration: 2000});
    })
  }

  editLecture() {
    this.isHidden = true;
    this.lectureService.updateLecture(new LectureModel(this.formEdit.controls['id'].value, this.formEdit.controls['name'].value,
      this.formEdit.controls['numberOfTimes'].value)).toPromise().then(() => {
      this.snackBar.open("Successfully updated lecture", "Close", {duration: 2000});
      this.isHidden = true;
      this.getLectures();
    }, () => {
      this.snackBar.open("Error updating lecture", "Close", {duration: 2000});
    })
  }

  editModeOn(id: number) {
    this.lectureService.getLecture(id).toPromise().then((res) => {
      this.formEdit.patchValue({
        id: res.id,
        name: res.name,
        numberOfTimes: res.number_of_times,

      });
      this.isHidden = false;
    }, () => {
      this.snackBar.open("Lecture with id doesn't exists.", "Close", {duration: 2000});
    })
  }

  getLectures() {
    this.lectureService.getLectures().toPromise().then((res) => {
      this.lectures = res;
    }, (err) => {
      this.snackBar.open("Server error: " + err.status, "Close", {duration: 2000});
    })
  }


}
