import {Injectable} from '@angular/core';
import {HttpClient} from '@angular/common/http';
import {Observable} from 'rxjs';
import {EntriesCount} from '../returns/entries-count';
import {Allocation} from '../relations/allocation';

@Injectable({
    providedIn: 'root'
  }
)
export class AllocationService {
  private apiUrl = "/api"

  constructor(private http: HttpClient) {
  }

  // get
  getAllocation(id: number): Observable<Allocation> {
    return this.http.get<Allocation>(this.apiUrl + "/allocations/" + id)
  }

  // multi get
  getAllocations(limit: number, page: number): Observable<Allocation[]> {
    return this.http.get<Allocation[]>(this.apiUrl + "/allocations?limit=" + limit + "&page=" + page + "")
  }

  // post or insert
  postAllocation(Allocation: Allocation): Observable<Allocation> {
    return this.http.post<Allocation>(this.apiUrl + "/allocations/", Allocation)
  }

  // patch or update
  patchAllocation(Allocation: Allocation): Observable<Allocation> {
    return this.http.patch<Allocation>(this.apiUrl + "/allocations/" + Allocation.id, Allocation);
  }

  deleteAllocation(id: number): Observable<Allocation> {
    return this.http.delete<Allocation>(this.apiUrl + "/allocations/" + id)
  }

  count(): Observable<EntriesCount> {
    return this.http.get<EntriesCount>(this.apiUrl + "/count/allocations");
  }
}
