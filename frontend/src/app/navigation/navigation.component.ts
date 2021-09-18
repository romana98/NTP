import {Component, OnInit} from '@angular/core';
import {Router} from '@angular/router';
import {UserRole} from '../model/user-role';
import {LogInService} from '../services/log-in.service';
import {StorageService} from '../services/storage.service';

@Component({
  selector: 'app-navigation',
  templateUrl: './navigation.component.html',
  styleUrls: ['./navigation.component.css']
})
export class NavigationComponent implements OnInit {

  role: UserRole = UserRole.UNAUTHORIZED;
  admin: UserRole = UserRole.ROLE_ADMINISTRATOR;
  staff: UserRole = UserRole.ROLE_STAFF;
  unauthorized: UserRole = UserRole.UNAUTHORIZED;

  constructor(private storageService: StorageService,
              private loginService: LogInService,
              private router: Router) {
    this.storageService.watchStorage().subscribe(() => {
      const user = JSON.parse(<string>localStorage.getItem('user'));

      if (user != null) {
        this.role = user.role === 'ROLE_ADMINISTRATOR' ? UserRole.ROLE_ADMINISTRATOR : (user.role === 'ROLE_STAFF' ? UserRole.ROLE_STAFF : UserRole.UNAUTHORIZED);
      } else {
        this.role = UserRole.UNAUTHORIZED
      }
    });

    const user = JSON.parse(<string>localStorage.getItem('user'));
    if (user != null) {
      this.role = user.role === 'ROLE_ADMINISTRATOR' ? UserRole.ROLE_ADMINISTRATOR : (user.role === 'ROLE_STAFF' ? UserRole.ROLE_STAFF : UserRole.UNAUTHORIZED);
    } else {
      this.role = UserRole.UNAUTHORIZED
    }
  }

  ngOnInit(): void {
  }

  logOut(): void {
    this.loginService.logOut();
    this.router.navigate(['/']);
  }

}
