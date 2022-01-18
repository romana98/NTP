import {Injectable} from '@angular/core';
import {HttpEvent, HttpHandler, HttpInterceptor, HttpRequest} from '@angular/common/http';
import {Observable} from 'rxjs';

@Injectable()
export class HttpAuthInterceptor implements HttpInterceptor {

  constructor() {
  }

  intercept(request: HttpRequest<unknown>, next: HttpHandler): Observable<HttpEvent<unknown>> {
    if (localStorage.getItem('user')) {
      request = request.clone({
        setHeaders: {
          Authorization: 'Bearer ' + JSON.parse(<string>localStorage.getItem('user')).accessToken
        }
      });
    }
    return next.handle(request);
  }
}

