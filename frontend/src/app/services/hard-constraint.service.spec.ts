import {TestBed} from '@angular/core/testing';

import {HardConstraintService} from './hard-constraint.service';

describe('HardConstraintService', () => {
  let service: HardConstraintService;

  beforeEach(() => {
    TestBed.configureTestingModule({});
    service = TestBed.inject(HardConstraintService);
  });

  it('should be created', () => {
    expect(service).toBeTruthy();
  });
});
