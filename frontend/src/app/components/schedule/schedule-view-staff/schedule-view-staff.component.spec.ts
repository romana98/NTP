import {ComponentFixture, TestBed} from '@angular/core/testing';

import {ScheduleViewStaffComponent} from './schedule-view-staff.component';

describe('ScheduleViewStaffComponent', () => {
  let component: ScheduleViewStaffComponent;
  let fixture: ComponentFixture<ScheduleViewStaffComponent>;

  beforeEach(async () => {
    await TestBed.configureTestingModule({
      declarations: [ScheduleViewStaffComponent]
    })
      .compileComponents();
  });

  beforeEach(() => {
    fixture = TestBed.createComponent(ScheduleViewStaffComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
