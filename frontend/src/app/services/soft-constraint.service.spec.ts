import {TestBed} from '@angular/core/testing';

import {SoftConstraintService} from './soft-constraint.service';

describe('SoftConstraintService', () => {
  let service: SoftConstraintService;

  beforeEach(() => {
    TestBed.configureTestingModule({});
    service = TestBed.inject(SoftConstraintService);
  });

  it('should be created', () => {
    expect(service).toBeTruthy();
  });
});
