import {Component} from '@angular/core';
import {ActivatedRoute, RouterLink} from '@angular/router';
import {take} from 'rxjs';
import {Transaction} from '../../relations/transaction';
import {TransactionService} from '../../services/transaction-service';

@Component({
  selector: 'app-transactions-component',
  imports: [
    RouterLink
  ],
  templateUrl: './transactions-component.html',
  styleUrl: './transactions-component.css'
})
export class TransactionsComponent {
  data: Transaction[] = [];
  elementCount: number = -1;
  table: string = "no-table";
  page: number = 0;
  entriesPerPage: number = 64;
  protected readonly Math = Math;

  constructor(private transactionService: TransactionService,
              private route: ActivatedRoute) {
  }

  ngOnInit(): void {
    const newPage: number = this.route.snapshot.params['page'];
    if (!(newPage === undefined || newPage === null)) {
      this.page = newPage;
    }

    this.transactionService.getTransactions(this.entriesPerPage, this.page).pipe(take(1)).subscribe((value) => {
      this.data = value;
    });
    this.transactionService.count().pipe(take(1)).subscribe((value) => {
      this.elementCount = value.count;
      this.table = value.table;
    });
  }
}
