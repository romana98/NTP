import {Injectable} from '@angular/core';
import {HttpClient} from "@angular/common/http";
import {Observable} from "rxjs";
import {LectureModel} from "../model/lecture-model";

@Injectable({
  providedIn: 'root'
})
export class LectureService {

  constructor(private http: HttpClient) {
  }

  getLecture(id: number): Observable<LectureModel> {
    return this.http.get<LectureModel>('http://localhost:8080/lectures/' + id)
  }

  getLectures(): Observable<any> {
    return this.http.get('http://localhost:8080/lectures')
  }

  getLecturesByStaff(): Observable<any> {
    return this.http.get('http://localhost:8080/lectures/by-staff')
  }

  getLecturesByFaculty(lectures: string[]): Observable<any> {
    return this.http.post('http://localhost:8080/lectures/by-faculty', lectures)
  }


  createLecture(lecture: LectureModel): Observable<LectureModel> {
    return this.http.post<LectureModel>('http://localhost:8080/lectures', lecture)
  }

  deleteLecture(id: number): Observable<String> {
    return this.http.delete<String>('http://localhost:8080/lectures/' + id)
  }

  updateLecture(lecture: LectureModel): Observable<LectureModel> {
    return this.http.put<LectureModel>('http://localhost:8080/lectures', lecture)
  }
}
