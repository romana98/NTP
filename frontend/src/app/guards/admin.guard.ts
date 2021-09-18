import {Injectable} from '@angular/core';
import {CanActivate, Router} from '@angular/router';
import {UserRole} from "../model/user-role";
import {LogInService} from "../services/log-in.service";

@Injectable({
  providedIn: 'root'
})
export class AdminGuard implements CanActivate {
  constructor(
    public auth: LogInService,
    public router: Router
  ) {
  }

  canActivate(): boolean {
    const role = this.auth.getRole();
    if (role !== UserRole.ROLE_ADMINISTRATOR) {
      this.router.navigate(['/']);
      return false;
    }

    return true;
  }

}

