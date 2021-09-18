import {Injectable} from '@angular/core';
import {HttpClient} from "@angular/common/http";
import {Observable} from "rxjs";
import {FacultyModel} from "../model/faculty-model";

@Injectable({
  providedIn: 'root'
})
export class FacultyService {

  constructor(private http: HttpClient) {
  }

  getFaculty(id: string): Observable<FacultyModel> {
    return this.http.get<FacultyModel>('http://localhost:8080/faculties/' + id)
  }

  getFaculties(): Observable<any> {
    return this.http.get('http://localhost:8080/faculties')
  }

  createFaculty(faculty: FacultyModel): Observable<FacultyModel> {
    return this.http.post<FacultyModel>('http://localhost:8080/faculties', faculty)
  }

  deleteFaculty(id: number): Observable<String> {
    return this.http.delete<String>('http://localhost:8080/faculties/' + id)
  }

  updateFaculty(faculty: FacultyModel): Observable<FacultyModel> {
    return this.http.put<FacultyModel>('http://localhost:8080/faculties', faculty)
  }
}
