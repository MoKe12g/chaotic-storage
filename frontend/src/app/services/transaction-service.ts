import {Injectable} from '@angular/core';
import {HttpClient} from '@angular/common/http';
import {Observable} from 'rxjs';
import {EntriesCount} from '../returns/entries-count';
import {Transaction} from '../relations/transaction';

@Injectable({
    providedIn: 'root'
  }
)
export class TransactionService {
  private apiUrl = "/api"

  constructor(private http: HttpClient) {
  }

  // get
  getTransaction(id: number): Observable<Transaction> {
    return this.http.get<Transaction>(this.apiUrl + "/transactions/" + id)
  }

  // multi get
  getTransactions(limit: number, page: number): Observable<Transaction[]> {
    return this.http.get<Transaction[]>(this.apiUrl + "/transactions?limit=" + limit + "&page=" + page + "")
  }

  // post or insert
  postTransaction(Transaction: Transaction): Observable<Transaction> {
    return this.http.post<Transaction>(this.apiUrl + "/transactions/", Transaction)
  }

  // patch or update
  patchTransaction(Transaction: Transaction): Observable<Transaction> {
    return this.http.patch<Transaction>(this.apiUrl + "/transactions/" + Transaction.id, Transaction);
  }

  deleteTransaction(id: number): Observable<Transaction> {
    return this.http.delete<Transaction>(this.apiUrl + "/transactions/" + id)
  }

  count(): Observable<EntriesCount> {
    return this.http.get<EntriesCount>(this.apiUrl + "/count/transactions");
  }
}
