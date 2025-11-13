import {ComponentFixture, TestBed} from '@angular/core/testing';

import {StorageBoxesComponent} from './storage-boxes-component';

describe('StorageBoxesComponent', () => {
  let component: StorageBoxesComponent;
  let fixture: ComponentFixture<StorageBoxesComponent>;

  beforeEach(async () => {
    await TestBed.configureTestingModule({
      imports: [StorageBoxesComponent]
    })
      .compileComponents();

    fixture = TestBed.createComponent(StorageBoxesComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
