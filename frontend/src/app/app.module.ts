import {NgModule} from '@angular/core';
import {BrowserModule} from '@angular/platform-browser';

import {AppComponent} from './app.component';
import {BrowserAnimationsModule} from '@angular/platform-browser/animations';
import {HTTP_INTERCEPTORS, HttpClientModule} from "@angular/common/http";
import {HttpAuthInterceptor} from "./interceptors/http-auth.interceptor";
import {AppRoutingModule} from "./app-routing/app-routing.module";
import {AuthModule} from "./components/auth/auth.module";
import {NavigationModule} from "./navigation/navigation.module";
import {SharedModule} from "./shared/shared.module";
import {ComponentsModule} from "./components/components.module";

@NgModule({
  declarations: [
    AppComponent
  ],
  imports: [
    BrowserModule,
    BrowserAnimationsModule,
    HttpClientModule,
    AppRoutingModule,
    ComponentsModule,
    SharedModule,
    NavigationModule,
    AuthModule
  ],
  providers: [{provide: HTTP_INTERCEPTORS, useClass: HttpAuthInterceptor, multi: true}],
  bootstrap: [AppComponent]
})
export class AppModule {
}
