import {ComponentFixture, TestBed} from '@angular/core/testing';

import {DatabaseView} from './database-view';

describe('DatabaseView', () => {
  let component: DatabaseView;
  let fixture: ComponentFixture<DatabaseView>;

  beforeEach(async () => {
    await TestBed.configureTestingModule({
      imports: [DatabaseView]
    })
      .compileComponents();

    fixture = TestBed.createComponent(DatabaseView);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
