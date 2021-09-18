import {Component, OnInit} from '@angular/core';
import {FormBuilder, FormGroup, Validators} from '@angular/forms';
import {MatSnackBar} from '@angular/material/snack-bar';
import {Router} from '@angular/router';
import {JwtHelperService} from '@auth0/angular-jwt';
import {LogInModel} from 'src/app/model/log-in-model';
import {LogIn} from 'src/app/model/user-role';
import {LogInService} from 'src/app/services/log-in.service';
import {StorageService} from 'src/app/services/storage.service';

@Component({
  selector: 'app-log-in',
  templateUrl: './log-in.component.html',
  styleUrls: ['./log-in.component.css']
})
export class LogInComponent implements OnInit {


  form: FormGroup;
  private fb: FormBuilder;

  constructor(
    fb: FormBuilder,
    public snackBar: MatSnackBar,
    private logInService: LogInService,
    public router: Router,
    private storageService: StorageService
  ) {
    this.fb = fb;
    this.form = this.fb.group({
      email: [null, [Validators.required, Validators.email]],
      password: [null, Validators.required]
    });
  }

  ngOnInit(): void {
  }

  submit(): void {
    const logIn = new LogIn(this.form.value.email, this.form.value.password);

    this.logInService.logIn(logIn).subscribe(
      result => {
        const jwt: JwtHelperService = new JwtHelperService();
        const info = jwt.decodeToken(result.accessToken);
        const role = info.role;
        const user = new LogInModel(info.email, result.accessToken, info.role);
        this.storageService.setStorageItem('user', JSON.stringify(user))

        this.snackBar.open('Successfully logged in!', 'Ok', {duration: 2000});

        if (role === 'ROLE_ADMINISTRATOR') {
          this.router.navigate(['/admin']);
        } else if (role === 'ROLE_STAFF') {
          this.router.navigate(['/staff']);
        }
      },
      error => {
        this.snackBar.open('Bad credentials!', 'Ok', {duration: 2000});
      }
    );
  }

}
