import {Component} from '@angular/core';
import {CategoryService} from '../../category-service/category-service';
import {EntriesCount} from '../../returns/entries-count';
import {take} from 'rxjs';

@Component({
  selector: 'app-dashboard-component',
  imports: [],
  templateUrl: './dashboard-component.html',
  styleUrl: './dashboard-component.css'
})
export class DashboardComponent {
  categoryCount: number = -1;


  constructor(private categoryService: CategoryService) {
    this.categoryService.count().pipe(
      take(1)
    ).subscribe((counter: EntriesCount) => {
      this.categoryCount = counter.count;
    });
  }

  ngOnInit(): void {
    console.log("test!");
  }
}
