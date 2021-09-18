import {Component, EventEmitter, OnInit, Output} from '@angular/core';
import {Router} from "@angular/router";

@Component({
  selector: 'app-navigation-admin',
  templateUrl: './navigation-admin.component.html',
  styleUrls: ['./navigation-admin.component.css']
})
export class NavigationAdminComponent implements OnInit {

  @Output() logOut = new EventEmitter<void>();

  constructor(public router: Router) {
  }

  ngOnInit(): void {
  }

  logOutAdmin(): void {
    this.logOut.emit();
  }

}
