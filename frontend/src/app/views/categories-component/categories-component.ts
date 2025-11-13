import {Component, OnInit} from '@angular/core';
import {CategoryService} from '../../services/category-service';
import {Category} from '../../relations/category';
import {take} from 'rxjs';
import {ActivatedRoute, RouterLink} from '@angular/router';

@Component({
  selector: 'app-categories-component',
  imports: [
    RouterLink
  ],
  templateUrl: './categories-component.html',
  styleUrl: './categories-component.css'
})
export class CategoriesComponent implements OnInit {
  data: Category[] = [];
  elementCount: number = -1;
  table: string = "no-table";
  page: number = 0;
  entriesPerPage: number = 64;
  protected readonly Math = Math;

  constructor(private categoryService: CategoryService,
              private route: ActivatedRoute) {
  }

  ngOnInit(): void {
    const newPage: number = this.route.snapshot.params['page'];
    if (!(newPage === undefined || newPage === null)) {
      this.page = newPage;
    }

    this.categoryService.getCategories(this.entriesPerPage, this.page).pipe(take(1)).subscribe((value) => {
      this.data = value;
    });
    this.categoryService.count().pipe(take(1)).subscribe((value) => {
      this.elementCount = value.count;
      this.table = value.table;
    });
  }
}
