import {ComponentFixture, TestBed} from '@angular/core/testing';

import {AllocationService} from './allocation-service';

describe('AllocationService', () => {
  let component: AllocationService;
  let fixture: ComponentFixture<AllocationService>;

  beforeEach(async () => {
    await TestBed.configureTestingModule({
      imports: [AllocationService]
    })
      .compileComponents();

    fixture = TestBed.createComponent(AllocationService);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
