import {Component, OnInit} from '@angular/core';
import {LectureService} from "../../../services/lecture.service";
import {MatSnackBar} from "@angular/material/snack-bar";

@Component({
  selector: 'app-lecture-view',
  templateUrl: './lecture-view.component.html',
  styleUrls: ['./lecture-view.component.css']
})
export class LectureViewComponent implements OnInit {

  lectures = []
  allLectures = []


  constructor(private lectureService: LectureService,
              public snackBar: MatSnackBar) {
  }

  ngOnInit(): void {
    this.getLectures();
  }

  getLectures() {
    this.lectureService.getLecturesByStaff().toPromise().then((res) => {
      this.lectures = res
    }, (err) => {
      this.snackBar.open("Server error: " + err.status, "Close", {duration: 2000});
    })
  }

}
