import {Component} from '@angular/core';
import {FormsModule} from '@angular/forms';
import {ActivatedRoute, Router} from '@angular/router';
import {take} from 'rxjs';
import {StorageBox} from '../../relations/storage-box';
import {StorageBoxService} from '../../services/storage-box-service';

@Component({
  selector: 'app-storage-boxes-edit-component',
  imports: [
    FormsModule
  ],
  templateUrl: './storage-boxes-edit-component.html',
  styleUrl: './storage-boxes-edit-component.css'
})
export class StorageBoxesEditComponent {
  storageBoxId: number = -1;
  storageBox: StorageBox;

  constructor(private storageBoxService: StorageBoxService,
              private route: ActivatedRoute,
              private router: Router,) {
    this.storageBox = new class implements StorageBox {
      place: string = "";
      item_type: number = -1;
      id = -1;
    };
  }

  ngOnInit(): void {
    this.storageBoxId = this.route.snapshot.params['id'];
    if (this.storageBoxId != -1) {
      this.getStorageBox(this.storageBoxId);
    }
    console.log('StorageBox ID:', this.storageBoxId);
  }

  getStorageBox(StorageBoxId: number) {
    this.storageBoxService.getStorageBox(StorageBoxId).pipe(take(1)).subscribe(
      (response) => {
        this.storageBox = response;
        console.log("replaced the StorageBox");
      }
    )
  }

  logContent() {
    console.log(this.storageBox.place);
  }

  postStorageBox() {
    this.storageBoxService.postStorageBox(this.storageBox).pipe(take(1)).subscribe({
      error: (e) => alert(e.message),
      next: (response) => {
        alert("HTTP Patch Request completed");
        this.storageBox = response;
        this.router.navigate(['/storage-box/' + response.id])
          .then(r => {
            if (!r) {
              alert("Redirection to storage-box page didn't work.")
            }
          });
      },
    });
    // TODO: this.StorageBoxService.postStorageBox()
  }

  patchStorageBox() {
    this.storageBoxService.patchStorageBox(this.storageBox).pipe(take(1)).subscribe({
      error: (e) => alert(e.message),
      next: (response) => {
        alert("HTTP Patch Request completed");
        this.storageBox = response;
      },
    });
  }

  deleteStorageBox() {
    this.storageBoxService.deleteStorageBox(this.storageBox.id).pipe(take(1)).subscribe({
      error: (e) => alert(e.message),
      next: (response) => {
        alert("HTTP Patch Request completed");
        this.router.navigate(['/storage-boxes']).then(r => {
          if (!r) {
            alert("Redirection to storage-boxes page didn't work.")
          }
        });
      },
    });
  }
}
