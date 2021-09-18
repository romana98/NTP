import {Injectable} from '@angular/core';
import {HttpClient} from "@angular/common/http";
import {Observable} from "rxjs";
import {HardConstraintModel} from "../model/hard-constraint-model";

@Injectable({
  providedIn: 'root'
})
export class HardConstraintService {


  constructor(private http: HttpClient) {
  }

  getHardConstraint(id: number): Observable<HardConstraintModel> {
    return this.http.get<HardConstraintModel>('http://localhost:8080/hard-constraints/' + id)
  }

  getHardConstraints(): Observable<any> {
    return this.http.get('http://localhost:8080/hard-constraints')
  }

  getHardConstraintsByProfessor(): Observable<any> {
    return this.http.get('http://localhost:8080/hard-constraints/by-professor')
  }

  createHardConstraint(hardConstraint: HardConstraintModel): Observable<HardConstraintModel> {
    return this.http.post<HardConstraintModel>('http://localhost:8080/hard-constraints', hardConstraint)
  }

  deleteHardConstraint(id: number): Observable<String> {
    return this.http.delete<String>('http://localhost:8080/hard-constraints/' + id)
  }

  updateHardConstraint(hardConstraint: HardConstraintModel): Observable<HardConstraintModel> {
    return this.http.put<HardConstraintModel>('http://localhost:8080/hard-constraints', hardConstraint)
  }
}
