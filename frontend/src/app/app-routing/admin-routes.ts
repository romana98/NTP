import {Routes} from "@angular/router";
import {AuthGuard} from "../guards/auth.guard";
import {AdminGuard} from "../guards/admin.guard";
import {StaffCrudComponent} from "../components/staff/staff-crud/staff-crud.component";
import {LectureCrudComponent} from "../components/lecture/lecture-crud/lecture-crud.component";
import {HardConstraintCrudComponent} from "../components/hard-constraint/hard-constraint-crud/hard-constraint-crud.component";
import {ShiftCrudComponent} from "../components/shift/shift-crud/shift-crud.component";
import {FacultyCrudComponent} from "../components/faculty/faculty-crud/faculty-crud.component";
import {ScheduleViewAdminComponent} from "../components/schedule/schedule-view-admin/schedule-view-admin.component";

export const adminRoutes: Routes = [
  {
    path: '',
    component: ScheduleViewAdminComponent,
    canActivate: [AuthGuard]
  },

  {
    path: 'staff',
    component: StaffCrudComponent,
    canActivate: [AdminGuard]
  },

  {
    path: 'lectures',
    component: LectureCrudComponent,
    canActivate: [AdminGuard]
  },

  {
    path: 'hard-constraints',
    component: HardConstraintCrudComponent,
    canActivate: [AdminGuard]
  },

  {
    path: 'shifts',
    component: ShiftCrudComponent,
    canActivate: [AdminGuard]
  },

  {
    path: 'faculties',
    component: FacultyCrudComponent,
    canActivate: [AdminGuard]
  },

  /* {
     path: 'view-profile',
     component: ViewProfileComponent,
     canActivate: [AuthGuard]
   },*/

];
