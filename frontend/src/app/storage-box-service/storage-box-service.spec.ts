import {ComponentFixture, TestBed} from '@angular/core/testing';

import {StorageBoxService} from './storage-box-service';

describe('StorageBoxService', () => {
  let component: StorageBoxService;
  let fixture: ComponentFixture<StorageBoxService>;

  beforeEach(async () => {
    await TestBed.configureTestingModule({
      imports: [StorageBoxService]
    })
      .compileComponents();

    fixture = TestBed.createComponent(StorageBoxService);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
