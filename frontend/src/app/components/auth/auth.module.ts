import {NgModule} from '@angular/core';
import {CommonModule} from '@angular/common';
import {LogInComponent} from "./log-in/log-in.component";
import {MatSnackBarModule} from "@angular/material/snack-bar";
import {MatDividerModule} from "@angular/material/divider";
import {MatFormFieldModule} from "@angular/material/form-field";
import {FormsModule, ReactiveFormsModule} from "@angular/forms";
import {MatInputModule} from "@angular/material/input";
import {MatButtonModule} from "@angular/material/button";


@NgModule({
  declarations: [LogInComponent],
  imports: [
    CommonModule,
    MatSnackBarModule,
    MatDividerModule,
    MatFormFieldModule,
    ReactiveFormsModule,
    FormsModule,
    MatInputModule,
    MatButtonModule
  ]
})
export class AuthModule {
}
