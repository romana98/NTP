import {Injectable} from '@angular/core';
import {CanActivate, Router} from '@angular/router';
import {LogInService} from "../services/log-in.service";
import {UserRole} from "../model/user-role";

@Injectable({
  providedIn: 'root'
})
export class AuthGuard implements CanActivate {
  constructor(
    public auth: LogInService,
    public router: Router
  ) {
  }

  canActivate(): boolean {
    const role = this.auth.getRole();
    if (role !== UserRole.ROLE_ADMINISTRATOR && role !== UserRole.ROLE_STAFF) {
      this.router.navigate(['/']);
      return false;
    }

    return true;
  }
}
