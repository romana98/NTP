import {ComponentFixture, TestBed} from '@angular/core/testing';

import {HardConstraintCrudComponent} from './hard-constraint-crud.component';

describe('HardConstraintCrudComponent', () => {
  let component: HardConstraintCrudComponent;
  let fixture: ComponentFixture<HardConstraintCrudComponent>;

  beforeEach(async () => {
    await TestBed.configureTestingModule({
      declarations: [HardConstraintCrudComponent]
    })
      .compileComponents();
  });

  beforeEach(() => {
    fixture = TestBed.createComponent(HardConstraintCrudComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
