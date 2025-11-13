import {ComponentFixture, TestBed} from '@angular/core/testing';

import {StorageBoxesEditComponent} from './storage-boxes-edit-component';

describe('StorageBoxesEditComponent', () => {
  let component: StorageBoxesEditComponent;
  let fixture: ComponentFixture<StorageBoxesEditComponent>;

  beforeEach(async () => {
    await TestBed.configureTestingModule({
      imports: [StorageBoxesEditComponent]
    })
      .compileComponents();

    fixture = TestBed.createComponent(StorageBoxesEditComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
