import {ComponentFixture, TestBed} from '@angular/core/testing';

import {AllocationsEditComponent} from './allocations-edit-component';

describe('AllocationsEditComponent', () => {
  let component: AllocationsEditComponent;
  let fixture: ComponentFixture<AllocationsEditComponent>;

  beforeEach(async () => {
    await TestBed.configureTestingModule({
      imports: [AllocationsEditComponent]
    })
      .compileComponents();

    fixture = TestBed.createComponent(AllocationsEditComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
