import {ComponentFixture, TestBed} from '@angular/core/testing';

import {ItemTypesEditComponent} from './item-types-edit-component';

describe('ItemTypesEditComponent', () => {
  let component: ItemTypesEditComponent;
  let fixture: ComponentFixture<ItemTypesEditComponent>;

  beforeEach(async () => {
    await TestBed.configureTestingModule({
      imports: [ItemTypesEditComponent]
    })
      .compileComponents();

    fixture = TestBed.createComponent(ItemTypesEditComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
