import {ComponentFixture, TestBed} from '@angular/core/testing';

import {ScheduleViewAdminComponent} from './schedule-view-admin.component';

describe('ScheduleViewComponent', () => {
  let component: ScheduleViewAdminComponent;
  let fixture: ComponentFixture<ScheduleViewAdminComponent>;

  beforeEach(async () => {
    await TestBed.configureTestingModule({
      declarations: [ScheduleViewAdminComponent]
    })
      .compileComponents();
  });

  beforeEach(() => {
    fixture = TestBed.createComponent(ScheduleViewAdminComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
