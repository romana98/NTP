import {Component, EventEmitter, OnInit, Output} from '@angular/core';
import {Router} from "@angular/router";

@Component({
  selector: 'app-navigation-staff',
  templateUrl: './navigation-staff.component.html',
  styleUrls: ['./navigation-staff.component.css']
})
export class NavigationStaffComponent implements OnInit {

  @Output() logOut = new EventEmitter<void>();

  constructor(public router: Router) {
  }

  ngOnInit(): void {
  }

  logOutProfessor(): void {
    this.logOut.emit();
  }

}
