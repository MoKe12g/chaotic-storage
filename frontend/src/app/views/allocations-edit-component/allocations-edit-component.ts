import {Component, OnInit} from '@angular/core';
import {FormsModule} from '@angular/forms';
import {ActivatedRoute, Router} from '@angular/router';
import {take} from 'rxjs';
import {Allocation} from '../../relations/allocation';
import {AllocationService} from '../../services/allocation-service';

@Component({
  selector: 'app-allocations-edit-component',
  imports: [
    FormsModule
  ],
  templateUrl: './allocations-edit-component.html',
  styleUrl: './allocations-edit-component.css'
})
export class AllocationsEditComponent implements OnInit {
  allocationId: number = -1;
  allocation: Allocation;

  constructor(private allocationService: AllocationService,
              private route: ActivatedRoute,
              private router: Router,) {
    this.allocation = new class implements Allocation {
      description: string = "";
      date_of_entry: Date = new Date();
      can_be_outside?: boolean = false;
      category_id: number = -1;
      storage_box_id: number = -1;
      id = -1;
    };
  }

  ngOnInit(): void {
    this.allocationId = this.route.snapshot.params['id'];
    if (this.allocationId != -1) {
      this.getAllocation(this.allocationId);
    }
    console.log('Allocation ID:', this.allocationId);
  }

  getAllocation(allocationId: number) {
    this.allocationService.getAllocation(allocationId).pipe(take(1)).subscribe(
      (response) => {
        this.allocation = response;
        console.log("replaced the allocation");
      }
    )
  }

  logContent() {
    console.log(this.allocation.description);
  }

  postAllocation() {
    this.allocationService.postAllocation(this.allocation).pipe(take(1)).subscribe({
      error: (e) => alert(e.message),
      next: (response) => {
        alert("HTTP Patch Request completed");
        this.allocation = response;
        this.router.navigate(['/category/' + response.id])
          .then(r => {
            if (!r) {
              alert("Redirection to categories page didn't work.")
            }
          });
      },
    });
    // TODO: this.categoryService.postAllocation()
  }

  patchAllocation() {
    this.allocationService.patchAllocation(this.allocation).pipe(take(1)).subscribe({
      error: (e) => alert(e.message),
      next: (response) => {
        alert("HTTP Patch Request completed");
        this.allocation = response;
      },
    });
  }

  deleteAllocation() {
    this.allocationService.deleteAllocation(this.allocation.id).pipe(take(1)).subscribe({
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
