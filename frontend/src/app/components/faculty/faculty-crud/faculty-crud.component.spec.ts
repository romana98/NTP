import {ComponentFixture, TestBed} from '@angular/core/testing';

import {FacultyCrudComponent} from './faculty-crud.component';

describe('FacultyCrudComponent', () => {
  let component: FacultyCrudComponent;
  let fixture: ComponentFixture<FacultyCrudComponent>;

  beforeEach(async () => {
    await TestBed.configureTestingModule({
      declarations: [FacultyCrudComponent]
    })
      .compileComponents();
  });

  beforeEach(() => {
    fixture = TestBed.createComponent(FacultyCrudComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
