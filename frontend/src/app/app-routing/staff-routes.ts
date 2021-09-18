import {Routes} from "@angular/router";
import {StaffGuard} from "../guards/staff.guard";
import {LectureViewComponent} from "../components/lecture/lecture-view/lecture-view.component";
import {SoftConstraintRuComponent} from "../components/soft-constraint/soft-constraint-ru/soft-constraint-ru.component";
import {ScheduleViewStaffComponent} from "../components/schedule/schedule-view-staff/schedule-view-staff.component";

export const staffRoutes: Routes = [
  {
    path: '',
    component: ScheduleViewStaffComponent,
    canActivate: [StaffGuard]
  },

  {
    path: 'lectures',
    component: LectureViewComponent,
    canActivate: [StaffGuard]
  },

  {
    path: 'soft-constraints',
    component: SoftConstraintRuComponent,
    canActivate: [StaffGuard]
  },
];
