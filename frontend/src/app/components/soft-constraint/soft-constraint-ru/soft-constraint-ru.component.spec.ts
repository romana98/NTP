import {ComponentFixture, TestBed} from '@angular/core/testing';

import {SoftConstraintRuComponent} from './soft-constraint-ru.component';

describe('SoftConstraintCruComponent', () => {
  let component: SoftConstraintRuComponent;
  let fixture: ComponentFixture<SoftConstraintRuComponent>;

  beforeEach(async () => {
    await TestBed.configureTestingModule({
      declarations: [SoftConstraintRuComponent]
    })
      .compileComponents();
  });

  beforeEach(() => {
    fixture = TestBed.createComponent(SoftConstraintRuComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
