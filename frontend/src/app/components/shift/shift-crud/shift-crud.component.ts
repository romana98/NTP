import {Component, OnInit, ViewChild} from '@angular/core';
import {FormBuilder, FormGroup, FormGroupDirective, Validators} from "@angular/forms";
import {MatSnackBar} from "@angular/material/snack-bar";
import {ShiftModel} from "../../../model/shift-model";
import {ShiftService} from "../../../services/shift.service";

@Component({
  selector: 'app-shift-crud',
  templateUrl: './shift-crud.component.html',
  styleUrls: ['./shift-crud.component.css']
})
export class ShiftCrudComponent implements OnInit {

  @ViewChild(FormGroupDirective) formGroupDirective: FormGroupDirective | undefined;

  shifts = []

  form: FormGroup;
  formEdit: FormGroup;
  isHidden = true;

  constructor(private fb: FormBuilder,
              private shiftService: ShiftService,
              public snackBar: MatSnackBar) {

    this.form = this.fb.group({
      id: [null, Validators.required],
      startHour: [null, Validators.required],
      startMinute: [null, Validators.required],
      endHour: [null, Validators.required],
      endMinute: [null, Validators.required],
      day: [null, Validators.required],

    });
    this.formEdit = this.fb.group({
      id: [null, Validators.required],
      startHour: [null, Validators.required],
      startMinute: [null, Validators.required],
      endHour: [null, Validators.required],
      endMinute: [null, Validators.required],
      day: [null, Validators.required],

    });
  }

  ngOnInit(): void {
    this.getShifts();
  }

  addShift() {
    this.shiftService.createShift(new ShiftModel("",
      this.form.controls['startHour'].value + ':' + this.form.controls['startMinute'].value,
      this.form.controls['endHour'].value + ':' + this.form.controls['endMinute'].value,
      this.form.controls['day'].value)).toPromise().then(() => {
      this.snackBar.open("Successfully created shift", "Close", {duration: 2000});
      this.getShifts();
      // @ts-ignore
      setTimeout(() => this.formGroupDirective.resetForm(), 0);
    }, () => {
      this.snackBar.open("Shift already exists.", "Close", {duration: 2000});
    })
  }

  deleteShift(id: number) {
    this.shiftService.deleteShift(id).toPromise().then(() => {
      this.snackBar.open("Successfully deleted shift", "Close", {duration: 2000});
      this.getShifts();
    }, () => {
      this.snackBar.open("Shift is part of a faculty.", "Close", {duration: 2000});
    })
  }

  editShift() {
    this.isHidden = true;
    this.shiftService.updateShift(new ShiftModel(this.formEdit.controls['id'].value,
      this.formEdit.controls['startHour'].value + ':' + this.formEdit.controls['startMinute'].value,
      this.formEdit.controls['endHour'].value + ':' + this.formEdit.controls['endMinute'].value,
      this.formEdit.controls['day'].value)).toPromise().then(() => {
      this.snackBar.open("Successfully updated shift", "Close", {duration: 2000});
      this.isHidden = true;
      this.getShifts();
    }, () => {
      this.snackBar.open("Error updating shift", "Close", {duration: 2000});
    })
  }

  editModeOn(id: number) {
    this.shiftService.getShift(id).toPromise().then((res) => {
      this.formEdit.patchValue({
        id: res.id,
        startHour: res.start.split(':')[0],
        startMinute: res.start.split(':')[1],
        endHour: res.end.split(':')[0],
        endMinute: res.end.split(':')[1],
        day: res.day

      });
      this.isHidden = false;
    }, () => {
      this.snackBar.open("Shift with id doesn't exists.", "Close", {duration: 2000});
    })
  }

  getShifts() {
    this.shiftService.getShifts().toPromise().then((res) => {
      this.shifts = res
    }, (err) => {
      this.snackBar.open("Server error: " + err.status, "Close", {duration: 2000});
    })
  }

}
