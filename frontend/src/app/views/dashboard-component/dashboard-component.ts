import {Component} from '@angular/core';
import {CategoryService} from '../../services/category-service';
import {EntriesCount} from '../../returns/entries-count';
import {take} from 'rxjs';
import {ItemTypeService} from '../../services/item-type-service';
import {AllocationService} from '../../services/allocation-service';
import {StorageBoxService} from '../../services/storage-box-service';
import {TransactionService} from '../../services/transaction-service';

@Component({
  selector: 'app-dashboard-component',
  imports: [],
  templateUrl: './dashboard-component.html',
  styleUrl: './dashboard-component.css'
})
export class DashboardComponent {
  protected categoryCount: number = -1;
  protected itemTypeCount: number = -1;
  protected storageBoxCount: number = -1;
  protected transactionCount: number = -1;
  protected allocationCount: number = -1;


  constructor(private categoryService: CategoryService,
              private itemTypeService: ItemTypeService,
              private allocationService: AllocationService,
              private storageBoxService: StorageBoxService,
              private transactionService: TransactionService) {
  }

  ngOnInit(): void {
    this.allocationService.count().pipe(
      take(1)
    ).subscribe((counter: EntriesCount) => {
      this.allocationCount = counter.count;
    });
    this.categoryService.count().pipe(
      take(1)
    ).subscribe((counter: EntriesCount) => {
      this.categoryCount = counter.count;
    });
    this.itemTypeService.count().pipe(
      take(1)
    ).subscribe((counter: EntriesCount) => {
      this.itemTypeCount = counter.count;
    });
    this.storageBoxService.count().pipe(
      take(1)
    ).subscribe((counter: EntriesCount) => {
      this.storageBoxCount = counter.count;
    });
    this.transactionService.count().pipe(
      take(1)
    ).subscribe((counter: EntriesCount) => {
      this.transactionCount = counter.count;
    });
  }
}
