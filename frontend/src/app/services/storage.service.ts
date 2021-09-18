import {Injectable} from '@angular/core';
import {Observable, Subject} from "rxjs";

@Injectable({
  providedIn: 'root'
})
export class StorageService {

  private storageSub = new Subject<string>();

  watchStorage(): Observable<any> {
    return this.storageSub.asObservable();
  }

  setStorageItem(key: string, data: any): void {
    localStorage.setItem(key, data);
    this.storageSub.next();
  }

  clearStorage(): void {
    localStorage.clear();
    this.storageSub.next();
  }
}
