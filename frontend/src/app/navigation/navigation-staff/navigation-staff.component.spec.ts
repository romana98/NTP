import {ComponentFixture, TestBed} from '@angular/core/testing';

import {NavigationStaffComponent} from './navigation-staff.component';

describe('NavigationProfessorComponent', () => {
  let component: NavigationStaffComponent;
  let fixture: ComponentFixture<NavigationStaffComponent>;

  beforeEach(async () => {
    await TestBed.configureTestingModule({
      declarations: [NavigationStaffComponent]
    })
      .compileComponents();
  });

  beforeEach(() => {
    fixture = TestBed.createComponent(NavigationStaffComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
