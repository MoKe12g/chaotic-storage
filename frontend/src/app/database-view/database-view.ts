import {Component, Injectable} from '@angular/core';
import {HttpClient} from '@angular/common/http';

@Component({
  selector: 'app-database-view',
  imports: [],
  templateUrl: './database-view.html',
  styleUrl: './database-view.css'
})

@Injectable({
  providedIn: 'root'
})

export class DatabaseView {

  constructor(private http: HttpClient) {

  }


}
