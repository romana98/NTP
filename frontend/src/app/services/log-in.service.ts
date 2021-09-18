import {Injectable} from '@angular/core';
import {HttpClient, HttpHeaders} from "@angular/common/http";
import {LogIn, UserRole} from "../model/user-role";
import {Observable} from "rxjs";
import {StorageService} from "./storage.service";

@Injectable({
  providedIn: 'root'
})
export class LogInService {

  private headers = new HttpHeaders({'Content-Type': 'application/json'});

  constructor(private http: HttpClient, private storageService: StorageService
  ) {
  }

  logIn(auth: LogIn): Observable<any> {
    return this.http.post('http://localhost:8080/auth/log-in',
      auth, {headers: this.headers, responseType: 'json'});
  }

  logOut(): void {
    this.storageService.clearStorage();
  }

  getRole(): UserRole {
    if (!localStorage.getItem('user')) {
      return UserRole.UNAUTHORIZED;
    }
    return JSON.parse(<string>localStorage.getItem('user')).role === 'ROLE_ADMINISTRATOR' ?
      UserRole.ROLE_ADMINISTRATOR : (JSON.parse(<string>localStorage.getItem('user')).role === 'ROLE_STAFF' ? UserRole.ROLE_STAFF : UserRole.UNAUTHORIZED);
  }
}
