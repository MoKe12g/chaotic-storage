import {Injectable} from '@angular/core';
import {HttpClient} from '@angular/common/http';
import {Observable} from 'rxjs';
import {EntriesCount} from '../returns/entries-count';
import {ItemType} from '../relations/item-type';

@Injectable({
    providedIn: 'root'
  }
)
export class ItemTypeService {
  private apiUrl = "/api"

  constructor(private http: HttpClient) {
  }

  // get
  getItemType(id: number): Observable<ItemType> {
    return this.http.get<ItemType>(this.apiUrl + "/item_types/" + id)
  }

  // multi get
  getItemTypes(limit: number, page: number): Observable<ItemType[]> {
    return this.http.get<ItemType[]>(this.apiUrl + "/item_types?limit=" + limit + "&page=" + page + "")
  }

  // post or insert
  postItemType(ItemType: ItemType): Observable<ItemType> {
    return this.http.post<ItemType>(this.apiUrl + "/item_types/", ItemType)
  }

  // patch or update
  patchItemType(ItemType: ItemType): Observable<ItemType> {
    return this.http.patch<ItemType>(this.apiUrl + "/item_types/" + ItemType.id, ItemType);
  }

  deleteItemType(id: number): Observable<ItemType> {
    return this.http.delete<ItemType>(this.apiUrl + "/item_types/" + id)
  }

  count(): Observable<EntriesCount> {
    return this.http.get<EntriesCount>(this.apiUrl + "/count/item_types");
  }
}
