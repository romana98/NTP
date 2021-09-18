import {NgModule} from '@angular/core';
import {CommonModule} from '@angular/common';
import {LectureCrudComponent} from './lecture/lecture-crud/lecture-crud.component';
import {LectureViewComponent} from './lecture/lecture-view/lecture-view.component';
import {ShiftCrudComponent} from './shift/shift-crud/shift-crud.component';
import {HardConstraintCrudComponent} from './hard-constraint/hard-constraint-crud/hard-constraint-crud.component';
import {SoftConstraintRuComponent} from './soft-constraint/soft-constraint-ru/soft-constraint-ru.component';
import {ScheduleViewAdminComponent} from './schedule/schedule-view-admin/schedule-view-admin.component';
import {StaffCrudComponent} from './staff/staff-crud/staff-crud.component';
import {MatDividerModule} from "@angular/material/divider";
import {MatInputModule} from "@angular/material/input";
import {MatSelectModule} from "@angular/material/select";
import {MatTooltipModule} from "@angular/material/tooltip";
import {ReactiveFormsModule} from "@angular/forms";
import {MatButtonModule} from "@angular/material/button";
import {SharedModule} from "../shared/shared.module";
import {FacultyCrudComponent} from './faculty/faculty-crud/faculty-crud.component';
import {ScheduleViewStaffComponent} from './schedule/schedule-view-staff/schedule-view-staff.component';


@NgModule({
  declarations: [
    LectureCrudComponent,
    LectureViewComponent,
    ShiftCrudComponent,
    HardConstraintCrudComponent,
    SoftConstraintRuComponent,
    ScheduleViewAdminComponent,
    StaffCrudComponent,
    FacultyCrudComponent,
    ScheduleViewStaffComponent
  ],
  imports: [
    CommonModule,
    MatDividerModule,
    MatInputModule,
    MatSelectModule,
    MatTooltipModule,
    ReactiveFormsModule,
    MatButtonModule,
    SharedModule
  ]
})
export class ComponentsModule {
}
