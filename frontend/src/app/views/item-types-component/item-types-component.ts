import {Component, OnInit} from '@angular/core';
import {ActivatedRoute, RouterLink} from '@angular/router';
import {take} from 'rxjs';
import {ItemType} from '../../relations/item-type';
import {ItemTypeService} from '../../services/item-type-service';

@Component({
  selector: 'app-item-types-component',
  imports: [
    RouterLink
  ],
  templateUrl: './item-types-component.html',
  styleUrl: './item-types-component.css'
})
export class ItemTypesComponent implements OnInit {
  data: ItemType[] = [];
  elementCount: number = -1;
  table: string = "no-table";
  page: number = 0;
  entriesPerPage: number = 64;
  protected readonly Math = Math;

  constructor(private itemTypeService: ItemTypeService,
              private route: ActivatedRoute) {
  }

  ngOnInit(): void {
    const newPage: number = this.route.snapshot.params['page'];
    if (!(newPage === undefined || newPage === null)) {
      this.page = newPage;
    }

    this.itemTypeService.getItemTypes(this.entriesPerPage, this.page).pipe(take(1)).subscribe((value) => {
      this.data = value;
    });
    this.itemTypeService.count().pipe(take(1)).subscribe((value) => {
      this.elementCount = value.count;
      this.table = value.table;
    });
  }
}
