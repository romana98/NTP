import {Injectable} from '@angular/core';
import {CanActivate, Router} from '@angular/router';
import {LogInService} from "../services/log-in.service";
import {UserRole} from "../model/user-role";

@Injectable({
  providedIn: 'root'
})
export class LogInGuard implements CanActivate {

  constructor(
    public auth: LogInService,
    public router: Router
  ) {
  }

  canActivate(): boolean {
    const role = this.auth.getRole();
    if (role === UserRole.ROLE_ADMINISTRATOR) {
      this.router.navigate(['/admin']);
      return false;
    } else if (role === UserRole.ROLE_STAFF) {
      this.router.navigate(['/staff']);
      return false;
    }

    return true;
  }

}
