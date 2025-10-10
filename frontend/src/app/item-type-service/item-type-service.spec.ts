import {ComponentFixture, TestBed} from '@angular/core/testing';

import {ItemTypeService} from './item-type-service';

describe('ItemTypeService', () => {
  let component: ItemTypeService;
  let fixture: ComponentFixture<ItemTypeService>;

  beforeEach(async () => {
    await TestBed.configureTestingModule({
      imports: [ItemTypeService]
    })
      .compileComponents();

    fixture = TestBed.createComponent(ItemTypeService);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
