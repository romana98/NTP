import {NgModule} from '@angular/core';
import {StaffGuard} from "../guards/staff.guard";
import {RouterModule, Routes} from "@angular/router";
import {adminRoutes} from "./admin-routes";
import {AdminGuard} from "../guards/admin.guard";
import {LogInGuard} from "../guards/log-in.guard";
import {staffRoutes} from "./staff-routes";
import {LogInComponent} from "../components/auth/log-in/log-in.component";


export const routes: Routes = [
  {
    path: '',
    component: LogInComponent,
    canActivate: [LogInGuard]
  },
  {
    path: 'admin',
    children: adminRoutes,
    canActivate: [AdminGuard]
  },
  {
    path: 'staff',
    children: staffRoutes,
    canActivate: [StaffGuard]
  },
  {
    path: '**',
    redirectTo: '/'
  }


];

@NgModule({
  imports: [RouterModule.forRoot(routes)],
  exports: [RouterModule]
})
export class AppRoutingModule {
}
