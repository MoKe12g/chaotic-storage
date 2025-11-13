import {Component, OnInit} from '@angular/core';
import {FormsModule} from '@angular/forms';
import {Transaction} from '../../relations/transaction';
import {TransactionService} from '../../services/transaction-service';
import {ActivatedRoute, Router} from '@angular/router';
import {take} from 'rxjs';

@Component({
  selector: 'app-transactions-edit-component',
  imports: [
    FormsModule
  ],
  templateUrl: './transactions-edit-component.html',
  styleUrl: './transactions-edit-component.css'
})
export class TransactionsEditComponent implements OnInit {
  transactionId: number = -1;
  transaction: Transaction;

  constructor(private transactionService: TransactionService,
              private route: ActivatedRoute,
              private router: Router,) {
    this.transaction = new class implements Transaction {
      allocation_id: number = -1;
      item_delta: number = 0;
      date: Date = new Date();
      id = -1;
    };
  }

  ngOnInit(): void {
    this.transactionId = this.route.snapshot.params['id'];
    if (this.transactionId != -1) {
      this.getTransaction(this.transactionId);
    }
    console.log('Transaction ID:', this.transactionId);
  }

  getTransaction(transactionId: number) {
    this.transactionService.getTransaction(transactionId).pipe(take(1)).subscribe(
      (response) => {
        this.transaction = response;
        console.log("replaced the transaction");
      }
    )
  }

  logContent() {
    console.log(this.transaction.allocation_id);
  }

  postTransaction() {
    this.transactionService.postTransaction(this.transaction).pipe(take(1)).subscribe({
      error: (e) => alert(e.message),
      next: (response) => {
        alert("HTTP POST Request completed");
        this.transaction = response;
        this.router.navigate(['/transaction/' + response.id])
          .then(r => {
            if (!r) {
              alert("Redirection to transactions page didn't work.")
            }
          });
      },
    });
  }

  patchTransaction() {
    this.transactionService.patchTransaction(this.transaction).pipe(take(1)).subscribe({
      error: (e) => alert(e.message),
      next: (response) => {
        alert("HTTP Patch Request completed");
        this.transaction = response;
      },
    });
  }

  deleteTransaction() {
    this.transactionService.deleteTransaction(this.transaction.id).pipe(take(1)).subscribe({
      error: (e) => alert(e.message),
      next: (response) => {
        alert("HTTP DELETE Request completed");
        this.router.navigate(['/transactions']).then(r => {
          if (!r) {
            alert("Redirection to transactions page didn't work.")
          }
        });
      },
    });
  }
}
