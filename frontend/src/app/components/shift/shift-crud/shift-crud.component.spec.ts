import {ComponentFixture, TestBed} from '@angular/core/testing';

import {ShiftCrudComponent} from './shift-crud.component';

describe('ShiftCrudComponent', () => {
  let component: ShiftCrudComponent;
  let fixture: ComponentFixture<ShiftCrudComponent>;

  beforeEach(async () => {
    await TestBed.configureTestingModule({
      declarations: [ShiftCrudComponent]
    })
      .compileComponents();
  });

  beforeEach(() => {
    fixture = TestBed.createComponent(ShiftCrudComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
