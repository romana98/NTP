import {Component, OnInit, ViewChild} from '@angular/core';
import {FormBuilder, FormGroup, FormGroupDirective, Validators} from "@angular/forms";
import {MatSnackBar} from "@angular/material/snack-bar";
import {SoftConstraintService} from "../../../services/soft-constraint.service";
import {SoftConstraintModel} from "../../../model/soft-constraint-model";

@Component({
  selector: 'app-soft-constraint-ru',
  templateUrl: './soft-constraint-ru.component.html',
  styleUrls: ['./soft-constraint-ru.component.css']
})
export class SoftConstraintRuComponent implements OnInit {
  @ViewChild(FormGroupDirective) formGroupDirective: FormGroupDirective | undefined;

  scs = []
  allSCs = []

  formEdit: FormGroup;
  isHidden = true;

  constructor(private fb: FormBuilder,
              private scService: SoftConstraintService,
              public snackBar: MatSnackBar) {


    this.formEdit = this.fb.group({
      id: [null, Validators.required],
      monday: [null, Validators.required],
      tuesday: [null, Validators.required],
      wednesday: [null, Validators.required],
      thursday: [null, Validators.required],
      friday: [null, Validators.required],

    });
  }

  ngOnInit(): void {
    this.getSC();
  }

  editSC() {
    this.isHidden = true;
    this.scService.updateSC(new SoftConstraintModel(this.allSCs[0]['id'], {
        monday: parseInt(this.formEdit.controls['monday'].value),
        tuesday: parseInt(this.formEdit.controls['tuesday'].value),
        wednesday: parseInt(this.formEdit.controls['wednesday'].value),
        thursday: parseInt(this.formEdit.controls['thursday'].value),
        friday: parseInt(this.formEdit.controls['friday'].value)
      }
    )).toPromise().then(() => {
      this.snackBar.open("Successfully updated shift", "Close", {duration: 2000});
      this.isHidden = true;
      this.getSC();
    }, () => {
      this.snackBar.open("Error updating shift", "Close", {duration: 2000});
    })
  }

  editModeOn() {
    this.formEdit.patchValue({
      id: this.allSCs[0]['id'],
      monday: this.allSCs[0]['prefers']['monday'],
      tuesday: this.allSCs[0]['prefers']['tuesday'],
      wednesday: this.allSCs[0]['prefers']['wednesday'],
      thursday: this.allSCs[0]['prefers']['thursday'],
      friday: this.allSCs[0]['prefers']['friday'],

    });
    this.isHidden = false;
  }

  getSC() {
    this.scService.getSC().toPromise().then((res) => {
      // @ts-ignore
      this.allSCs = [res]
      // @ts-ignore
      this.scs = this.allSCs.map(item => item.prefers)

    }, (err) => {
      this.snackBar.open("Server error: " + err.status, "Close", {duration: 2000});
    })
  }

}
