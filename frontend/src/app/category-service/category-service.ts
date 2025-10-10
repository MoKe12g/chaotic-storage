import {Component, Injectable} from '@angular/core';
import {HttpClient} from '@angular/common/http';
import {Observable} from 'rxjs';
import {Category} from '../relations/category';
import {EntriesCount} from '../returns/entries-count';

@Component({
  selector: 'app-category-service',
  imports: [],
  templateUrl: './category-service.html',
  styleUrl: './category-service.css'
})

@Injectable({
    providedIn: 'root'
  }
)

export class CategoryService {
  private apiUrl = "/api"

  constructor(private http: HttpClient) {
  }

  // get
  getCategory(id: number): Observable<Category> {
    return this.http.get<Category>(this.apiUrl + "/categories/" + id)
  }

  // multi get
  getCategories(limit: number, page: number): Observable<Category[]> {
    return this.http.get<Category[]>(this.apiUrl + "/categories?limit=" + limit + "&page=" + page + "")
  }

  // post or insert
  postCategory(category: Category): Observable<Category> {
    return this.http.post<Category>(this.apiUrl + "/categories/", category)
  }

  // patch or update
  patchCategory(category: Category): Observable<Category> {
    return this.http.patch<Category>(this.apiUrl + "/categories/" + category.id, category);
  }

  deleteCategory(id: number): Observable<Category> {
    return this.http.delete<Category>(this.apiUrl + "/categories/" + id)
  }

  count(): Observable<EntriesCount> {
    return this.http.get<EntriesCount>(this.apiUrl + "/count/categories");
  }
}
