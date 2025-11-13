import {Component, OnInit} from '@angular/core';
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
export class TransactionsComponent implements OnInit {
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
    const newPageParam = this.route.snapshot.params['page'];
    if (newPageParam !== undefined) {
      const parsed = Number(newPageParam)
      if (!Number.isNaN(parsed)) this.page = parsed;
    }

    this.transactionService.getTransactions(this.entriesPerPage, this.page).pipe(take(1)).subscribe((value) => {
      this.data = value;
    });
    this.transactionService.count().pipe(take(1)).subscribe((value) => {
      this.elementCount = value.count;
      this.table = value.table;
    }, (error) => {
      console.error('Failed to load transaction count:', error);
      this.elementCount = 0;
    });
  }
}
