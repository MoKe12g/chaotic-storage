import {Component, signal} from '@angular/core';
import {MenuComponent} from './views/menu-component/menu-component';
import {RouterOutlet} from '@angular/router';

@Component({
  selector: 'app-root',
  imports: [MenuComponent, RouterOutlet],
  templateUrl: './app.html',
  //providers: [HttpClient],
  styleUrl: './app.css'
})

export class App {
  protected readonly title = signal('chaotic-storage-frontend');
}
