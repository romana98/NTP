import {Injectable} from '@angular/core';
import {HttpClient} from "@angular/common/http";
import {Observable} from "rxjs";
import {StaffModel} from "../model/staff-model";

@Injectable({
  providedIn: 'root'
})
export class StaffService {
  constructor(private http: HttpClient) {
  }

  getStaff(id: number): Observable<StaffModel> {
    return this.http.get<StaffModel>('http://localhost:8080/staff/' + id)
  }

  getStaffList(): Observable<any> {
    return this.http.get('http://localhost:8080/staff')
  }

  createStaff(staff: StaffModel): Observable<StaffModel> {
    return this.http.post<StaffModel>('http://localhost:8080/staff', staff)
  }

  deleteStaff(id: number): Observable<String> {
    return this.http.delete<String>('http://localhost:8080/staff/' + id)
  }

  updateStaff(staff: StaffModel): Observable<StaffModel> {
    return this.http.put<StaffModel>('http://localhost:8080/staff', staff)
  }
}
