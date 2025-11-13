import {Component, OnInit} from '@angular/core';
import {CategoryService} from '../../services/category-service';
import {ActivatedRoute, Router} from '@angular/router';
import {Category} from '../../relations/category';
import {take} from 'rxjs';
import {FormsModule} from '@angular/forms';

@Component({
  selector: 'app-categories-edit-component',
  imports: [
    FormsModule
  ],
  templateUrl: './categories-edit-component.html',
  styleUrl: './categories-edit-component.css'
})
export class CategoriesEditComponent implements OnInit {
  categoryId: number = -1;
  category: Category;

  constructor(private categoryService: CategoryService,
              private route: ActivatedRoute,
              private router: Router,) {
    this.category = new class implements Category {
      comment = "";
      id = -1;
    };
  }

  ngOnInit(): void {
    this.categoryId = this.route.snapshot.params['id'];
    if (this.categoryId != -1) {
      this.getCategory(this.categoryId);
    }
    console.log('Category ID:', this.categoryId);
  }

  getCategory(categoryId: number) {
    this.categoryService.getCategory(categoryId).pipe(take(1)).subscribe(
      (response) => {
        this.category = response;
        console.log("replaced the category");
      }
    )
  }

  logContent() {
    console.log(this.category.comment);
  }

  postCategory() {
    this.categoryService.postCategory(this.category).pipe(take(1)).subscribe({
      error: (e) => alert(e.message),
      next: (response) => {
        alert("HTTP Patch Request completed");
        this.category = response;
        this.router.navigate(['/category/' + response.id])
          .then(r => {
            if (!r) {
              alert("Redirection to categories page didn't work.")
            }
          });
      },
    });
    // TODO: this.categoryService.postCategory()
  }

  patchCategory() {
    this.categoryService.patchCategory(this.category).pipe(take(1)).subscribe({
      error: (e) => alert(e.message),
      next: (response) => {
        alert("HTTP Patch Request completed");
        this.category = response;
      },
    });
  }

  deleteCategory() {
    this.categoryService.deleteCategory(this.category.id).pipe(take(1)).subscribe({
      error: (e) => alert(e.message),
      next: (response) => {
        alert("HTTP Patch Request completed");
        this.router.navigate(['/categories']).then(r => {
          if (!r) {
            alert("Redirection to categories page didn't work.")
          }
        });
      },
    });
  }
}
