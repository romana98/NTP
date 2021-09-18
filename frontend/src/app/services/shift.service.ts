import {Injectable} from '@angular/core';
import {ShiftModel} from "../model/shift-model";
import {Observable} from "rxjs";
import {HttpClient} from "@angular/common/http";

@Injectable({
  providedIn: 'root'
})
export class ShiftService {

  constructor(private http: HttpClient) {
  }

  getShift(id: number): Observable<ShiftModel> {
    return this.http.get<ShiftModel>('http://localhost:8080/shifts/' + id)
  }

  getShifts(): Observable<any> {
    return this.http.get('http://localhost:8080/shifts')
  }

  createShift(shift: ShiftModel): Observable<ShiftModel> {
    return this.http.post<ShiftModel>('http://localhost:8080/shifts', shift)
  }

  deleteShift(id: number): Observable<String> {
    return this.http.delete<String>('http://localhost:8080/shifts/' + id)
  }

  updateShift(shift: ShiftModel): Observable<ShiftModel> {
    return this.http.put<ShiftModel>('http://localhost:8080/shifts', shift)
  }

}
