import {ComponentFixture, TestBed} from '@angular/core/testing';

import {NavigationNotLoggedInComponent} from './navigation-not-logged-in.component';

describe('NavigationNotLoggedInComponent', () => {
  let component: NavigationNotLoggedInComponent;
  let fixture: ComponentFixture<NavigationNotLoggedInComponent>;

  beforeEach(async () => {
    await TestBed.configureTestingModule({
      declarations: [NavigationNotLoggedInComponent]
    })
      .compileComponents();
  });

  beforeEach(() => {
    fixture = TestBed.createComponent(NavigationNotLoggedInComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
