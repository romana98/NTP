import {Component, OnInit, ViewChild} from '@angular/core';
import {FormBuilder, FormGroup, FormGroupDirective, Validators} from "@angular/forms";
import {MatSnackBar} from "@angular/material/snack-bar";
import {HardConstraintService} from "../../../services/hard-constraint.service";
import {HardConstraintModel} from "../../../model/hard-constraint-model";

@Component({
  selector: 'app-hard-constraint-crud',
  templateUrl: './hard-constraint-crud.component.html',
  styleUrls: ['./hard-constraint-crud.component.css']
})
export class HardConstraintCrudComponent implements OnInit {

  @ViewChild(FormGroupDirective) formGroupDirective: FormGroupDirective | undefined;

  hardConstraints = []

  form: FormGroup;
  formEdit: FormGroup;
  isHidden = true;

  constructor(private fb: FormBuilder,
              private hardConstraintService: HardConstraintService,
              public snackBar: MatSnackBar) {

    this.form = this.fb.group({
      id: [null, Validators.required],
      dailyMax: [null, Validators.required],
      weeklyMax: [null, Validators.required],
      weeklyMin: [null, Validators.required],
      maxPerShift: [null, Validators.required]

    });
    this.formEdit = this.fb.group({
      id: [null, Validators.required],
      dailyMax: [null, Validators.required],
      weeklyMax: [null, Validators.required],
      weeklyMin: [null, Validators.required],
      maxPerShift: [null, Validators.required]

    });
  }

  ngOnInit(): void {
    this.getHardConstraints();
  }


  addHardConstraint() {
    this.hardConstraintService.createHardConstraint(new HardConstraintModel("",
      this.form.controls['dailyMax'].value, this.form.controls['weeklyMax'].value, this.form.controls['weeklyMin'].value, this.form.controls['maxPerShift'].value)).toPromise().then(() => {
      this.snackBar.open("Successfully created hard constraint", "Close", {duration: 2000});
      this.getHardConstraints();
      // @ts-ignore
      setTimeout(() => this.formGroupDirective.resetForm(), 0);
    }, () => {
      this.snackBar.open("Hard Constraint already exists.", "Close", {duration: 2000});
    })
  }

  deleteHardConstraint(id: number) {
    this.hardConstraintService.deleteHardConstraint(id).toPromise().then(() => {
      this.snackBar.open("Successfully deleted hard constraint", "Close", {duration: 2000});
      this.getHardConstraints();
    }, () => {
      this.snackBar.open("Hard Constraint is part of a faculty.", "Close", {duration: 2000});
    })
  }

  editHardConstraint() {
    this.isHidden = true;
    this.hardConstraintService.updateHardConstraint(new HardConstraintModel(this.formEdit.controls['id'].value,
      this.formEdit.controls['dailyMax'].value,
      this.formEdit.controls['weeklyMax'].value,
      this.formEdit.controls['weeklyMin'].value,
      this.formEdit.controls['maxPerShift'].value)).toPromise().then(() => {
      this.snackBar.open("Successfully updated hard constraint", "Close", {duration: 2000});
      this.isHidden = true;
      this.getHardConstraints();
    }, () => {
      this.snackBar.open("Error updating hard constraint", "Close", {duration: 2000});
    })
  }

  editModeOn(id: number) {
    this.hardConstraintService.getHardConstraint(id).toPromise().then((res) => {
      console.log(res)
      this.formEdit.patchValue({
        id: res.id,
        dailyMax: res.daily_max,
        weeklyMax: res.weekly_max,
        weeklyMin: res.weekly_min,
        maxPerShift: res.max_per_shift

      });
      this.isHidden = false;
    }, () => {
      this.snackBar.open("Hard Constraint with id doesn't exists.", "Close", {duration: 2000});
    })
  }

  getHardConstraints() {
    this.hardConstraintService.getHardConstraints().toPromise().then((res) => {

      this.hardConstraints = res
    }, (err) => {
      this.snackBar.open("Server error: " + err.status, "Close", {duration: 2000});
    })
  }

}
