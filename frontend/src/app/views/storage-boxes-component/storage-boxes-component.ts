import {Component, OnInit} from '@angular/core';
import {ActivatedRoute, RouterLink} from '@angular/router';
import {take} from 'rxjs';
import {StorageBox} from '../../relations/storage-box';
import {StorageBoxService} from '../../services/storage-box-service';

@Component({
  selector: 'app-storage-boxes-component',
  imports: [
    RouterLink
  ],
  templateUrl: './storage-boxes-component.html',
  styleUrl: './storage-boxes-component.css'
})
export class StorageBoxesComponent implements OnInit {
  data: StorageBox[] = [];
  elementCount: number = -1;
  table: string = "no-table";
  page: number = 0;
  entriesPerPage: number = 64;
  protected readonly Math = Math;

  constructor(private storageBoxService: StorageBoxService,
              private route: ActivatedRoute) {
  }

  ngOnInit(): void {
    const newPageParam: number = this.route.snapshot.params['page'];
    if (newPageParam !== null) {
      const parsed = Number(newPageParam)
      if (!Number.isNaN(parsed)) this.page = newPageParam;
    }

    this.storageBoxService.getStorageBoxes(this.entriesPerPage, this.page).pipe(take(1)).subscribe((value) => {
      this.data = value;
    });
    this.storageBoxService.count().pipe(take(1)).subscribe((value) => {
      this.elementCount = value.count;
      this.table = value.table;
    }, (error) => {
      console.error('Failed to load storage box count:', error);
      this.elementCount = 0;
    });
  }
}
