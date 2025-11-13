import {Component} from '@angular/core';
import {ActivatedRoute, Router} from '@angular/router';
import {take} from 'rxjs';
import {FormsModule} from '@angular/forms';
import {ItemType} from '../../relations/item-type';
import {ItemTypeService} from '../../services/item-type-service';

@Component({
  selector: 'app-item-types-edit-component',
  imports: [
    FormsModule
  ],
  templateUrl: './item-types-edit-component.html',
  styleUrl: './item-types-edit-component.css'
})
export class ItemTypesEditComponent {
  itemTypeId: number = -1;
  itemType: ItemType;

  constructor(private itemTypeService: ItemTypeService,
              private route: ActivatedRoute,
              private router: Router,) {
    this.itemType = new class implements ItemType {
      storage_property = "";
      id = -1;
    };
  }

  ngOnInit(): void {
    this.itemTypeId = this.route.snapshot.params['id'];
    if (this.itemTypeId != -1) {
      this.getCategory(this.itemTypeId);
    }
    console.log('Category ID:', this.itemTypeId);
  }

  getCategory(categoryId: number) {
    this.itemTypeService.getItemType(categoryId).pipe(take(1)).subscribe(
      (response) => {
        this.itemType = response;
        console.log("replaced the item type");
      }
    )
  }

  logContent() {
    console.log(this.itemType.storage_property);
  }

  postCategory() {
    this.itemTypeService.postItemType(this.itemType).pipe(take(1)).subscribe({
      error: (e) => alert(e.message),
      next: (response) => {
        alert("HTTP Patch Request completed");
        this.itemType = response;
        this.router.navigate(['/item-type/' + response.id])
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
    this.itemTypeService.patchItemType(this.itemType).pipe(take(1)).subscribe({
      error: (e) => alert(e.message),
      next: (response) => {
        alert("HTTP Patch Request completed");
        this.itemType = response;
      },
    });
  }

  deleteCategory() {
    this.itemTypeService.deleteItemType(this.itemType.id).pipe(take(1)).subscribe({
      error: (e) => alert(e.message),
      next: (response) => {
        alert("HTTP Patch Request completed");
        this.router.navigate(['/item-types']).then(r => {
          if (!r) {
            alert("Redirection to item types page didn't work.")
          }
        });
      },
    });
  }
}
