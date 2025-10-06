import {Routes} from '@angular/router';
import {AllocationsComponent} from './allocations-component/allocations-component';
import {StorageBoxesComponent} from './storage-boxes-component/storage-boxes-component';
import {CategoriesComponent} from './categories-component/categories-component';
import {TransactionsComponent} from './transactions-component/transactions-component';
import {ItemTypesComponent} from './item-types-component/item-types-component';

export const routes: Routes = [
  // TODO: Add default path {path: '', redirectTo: '/storage_boxes', pathMatch: 'full'},
  {path: 'storage-boxes/:id', component: StorageBoxesComponent},
  {path: 'categories/:id', component: CategoriesComponent},
  {path: 'allocations/:id', component: AllocationsComponent},
  {path: 'transactions/:id', component: TransactionsComponent},
  {path: 'item-types/:id', component: ItemTypesComponent},
];
