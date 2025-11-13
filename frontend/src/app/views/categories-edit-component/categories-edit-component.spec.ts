import {ComponentFixture, TestBed} from '@angular/core/testing';

import {CategoriesEditComponent} from './categories-edit-component';

describe('CategoriesEditComponent', () => {
  let component: CategoriesEditComponent;
  let fixture: ComponentFixture<CategoriesEditComponent>;

  beforeEach(async () => {
    await TestBed.configureTestingModule({
      imports: [CategoriesEditComponent]
    })
      .compileComponents();

    fixture = TestBed.createComponent(CategoriesEditComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
