import {Component, OnInit} from '@angular/core';
import {ActivatedRoute, RouterLink} from '@angular/router';
import {take} from 'rxjs';
import {Allocation} from '../../relations/allocation';
import {AllocationService} from '../../services/allocation-service';

@Component({
  selector: 'app-allocations-component',
  imports: [
    RouterLink
  ],
  templateUrl: './allocations-component.html',
  styleUrl: './allocations-component.css'
})
export class AllocationsComponent implements OnInit {
  data: Allocation[] = [];
  elementCount: number = -1;
  table: string = "no-table";
  page: number = 0;
  entriesPerPage: number = 64;
  protected readonly Math = Math;

  constructor(private allocationService: AllocationService,
              private route: ActivatedRoute) {
  }

  ngOnInit(): void {
    const newPage: number = this.route.snapshot.params['page'];
    if (!(newPage === undefined || newPage === null)) {
      this.page = newPage;
    }

    this.allocationService.getAllocations(this.entriesPerPage, this.page).pipe(take(1)).subscribe((value) => {
      this.data = value;
    });
    this.allocationService.count().pipe(take(1)).subscribe((value) => {
      this.elementCount = value.count;
      this.table = value.table;
    });
  }
}
