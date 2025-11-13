import {Injectable} from '@angular/core';
import {HttpClient} from '@angular/common/http';
import {Observable} from 'rxjs';
import {EntriesCount} from '../returns/entries-count';
import {StorageBox} from '../relations/storage-box';

@Injectable({
    providedIn: 'root'
  }
)
export class StorageBoxService {
  private apiUrl = "/api"

  constructor(private http: HttpClient) {
  }

  // get
  getStorageBox(id: number): Observable<StorageBox> {
    return this.http.get<StorageBox>(this.apiUrl + "/storage_boxes/" + id)
  }

  // multi get
  getStorageBoxes(limit: number, page: number): Observable<StorageBox[]> {
    return this.http.get<StorageBox[]>(this.apiUrl + "/storage_boxes?limit=" + limit + "&page=" + page + "")
  }

  // post or insert
  postStorageBox(StorageBox: StorageBox): Observable<StorageBox> {
    return this.http.post<StorageBox>(this.apiUrl + "/storage_boxes/", StorageBox)
  }

  // patch or update
  patchStorageBox(StorageBox: StorageBox): Observable<StorageBox> {
    return this.http.patch<StorageBox>(this.apiUrl + "/storage_boxes/" + StorageBox.id, StorageBox);
  }

  deleteStorageBox(id: number): Observable<StorageBox> {
    return this.http.delete<StorageBox>(this.apiUrl + "/storage_boxes/" + id)
  }

  count(): Observable<EntriesCount> {
    return this.http.get<EntriesCount>(this.apiUrl + "/count/storage_boxes");
  }
}
