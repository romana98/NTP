import {Component, OnInit} from '@angular/core';
import {ScheduleService} from "../../../services/schedule.service";
import {MatSnackBar} from "@angular/material/snack-bar";

@Component({
  selector: 'app-schedule-view-staff',
  templateUrl: './schedule-view-staff.component.html',
  styleUrls: ['./schedule-view-staff.component.css']
})
export class ScheduleViewStaffComponent implements OnInit {

  schedule = []

  constructor(private scheduleService: ScheduleService,
              public snackBar: MatSnackBar) {
  }

  ngOnInit(): void {
    this.getSchedule();
  }

  getSchedule() {
    this.scheduleService.getScheduleByStaff().toPromise().then((res) => {
      this.schedule = res

    }, (err) => {
      this.snackBar.open("Faculty doesn't have a schedule", "Close", {duration: 2000});
    })
  }

}
