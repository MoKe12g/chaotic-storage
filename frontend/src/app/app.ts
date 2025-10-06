import {Component, signal} from '@angular/core';
import {MenuComponent} from './menu-component/menu-component';

@Component({
  selector: 'app-root',
  imports: [MenuComponent],
  templateUrl: './app.html',
  styleUrl: './app.css'
})
export class App {
  protected readonly title = signal('chaotic-storage-frontend');
}
