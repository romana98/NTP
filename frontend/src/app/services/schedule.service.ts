import {Injectable} from '@angular/core';
import {Observable} from "rxjs";
import {HttpClient} from "@angular/common/http";

@Injectable({
  providedIn: 'root'
})
export class ScheduleService {

  constructor(private http: HttpClient) {
  }

  generateSchedule(id: { id: string }): Observable<any> {
    return this.http.post('http://localhost:8080/schedule', id)
  }

  getSchedule(id: number): Observable<any> {
    return this.http.get('http://localhost:8080/schedule/' + id)
  }

  getScheduleByStaff(): Observable<any> {
    return this.http.get('http://localhost:8080/schedule/by-staff')
  }
}
