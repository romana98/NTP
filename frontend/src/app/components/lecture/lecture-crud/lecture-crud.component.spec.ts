import {ComponentFixture, TestBed} from '@angular/core/testing';

import {LectureCrudComponent} from './lecture-crud.component';

describe('LectureCrudComponent', () => {
  let component: LectureCrudComponent;
  let fixture: ComponentFixture<LectureCrudComponent>;

  beforeEach(async () => {
    await TestBed.configureTestingModule({
      declarations: [LectureCrudComponent]
    })
      .compileComponents();
  });

  beforeEach(() => {
    fixture = TestBed.createComponent(LectureCrudComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
