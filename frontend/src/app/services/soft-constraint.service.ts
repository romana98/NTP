import {Injectable} from '@angular/core';
import {HttpClient} from "@angular/common/http";
import {Observable} from "rxjs";
import {SoftConstraintModel} from "../model/soft-constraint-model";

@Injectable({
  providedIn: 'root'
})
export class SoftConstraintService {

  constructor(private http: HttpClient) {
  }

  getSC(): Observable<any> {
    return this.http.get('http://localhost:8080/soft-constraints')
  }

  updateSC(sc: SoftConstraintModel): Observable<SoftConstraintModel> {
    return this.http.put<SoftConstraintModel>('http://localhost:8080/soft-constraints', sc)
  }

}
