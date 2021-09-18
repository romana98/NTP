import {NgModule} from '@angular/core';
import {CommonModule} from '@angular/common';
import {NavigationComponent} from './navigation.component';
import {MatIconModule} from "@angular/material/icon";
import {MatToolbarModule} from "@angular/material/toolbar";
import {AppRoutingModule} from "../app-routing/app-routing.module";
import {MatButtonModule} from "@angular/material/button";
import {NavigationNotLoggedInComponent} from './navigation-not-logged-in/navigation-not-logged-in.component';
import {NavigationAdminComponent} from './navigation-admin/navigation-admin.component';
import {NavigationStaffComponent} from './navigation-staff/navigation-staff.component';


@NgModule({
  declarations: [
    NavigationComponent,
    NavigationNotLoggedInComponent,
    NavigationAdminComponent,
    NavigationStaffComponent
  ],
  imports: [
    CommonModule,
    MatIconModule,
    MatToolbarModule,
    MatButtonModule,
    AppRoutingModule
  ],
  exports: [
    NavigationComponent,
    NavigationNotLoggedInComponent,
    NavigationAdminComponent,
    NavigationStaffComponent
  ]
})
export class NavigationModule {
}
