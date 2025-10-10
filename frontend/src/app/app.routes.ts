import {Routes} from '@angular/router';
import {AllocationsComponent} from './views/allocations-component/allocations-component';
import {StorageBoxesComponent} from './views/storage-boxes-component/storage-boxes-component';
import {CategoriesComponent} from './views/categories-component/categories-component';
import {TransactionsComponent} from './views/transactions-component/transactions-component';
import {ItemTypesComponent} from './views/item-types-component/item-types-component';
import {DashboardComponent} from './views/dashboard-component/dashboard-component';
import {StorageBoxesEditComponent} from './views/storage-boxes-edit-component/storage-boxes-edit-component';
import {ItemTypesEditComponent} from './views/item-types-edit-component/item-types-edit-component';
import {AllocationsEditComponent} from './views/allocations-edit-component/allocations-edit-component';
import {CategoriesEditComponent} from './views/categories-edit-component/categories-edit-component';
import {TransactionsEditComponent} from './views/transactions-edit-component/transactions-edit-component';

export const routes: Routes = [
  {path: '', redirectTo: '/dashboard', pathMatch: 'full'},
  {path: 'dashboard', component: DashboardComponent},
  {path: 'storage-boxes', component: StorageBoxesComponent},
  {path: 'storage-boxes/:page', component: StorageBoxesComponent},
  {path: 'categories', component: CategoriesComponent},
  {path: 'categories/:page', component: CategoriesComponent},
  {path: 'allocations', component: AllocationsComponent},
  {path: 'allocations/:page', component: AllocationsComponent},
  {path: 'transactions', component: TransactionsComponent},
  {path: 'transactions/:page', component: TransactionsComponent},
  {path: 'item-types', component: ItemTypesComponent},
  {path: 'item-types/:page', component: ItemTypesComponent},
  {path: 'storage-box/:id', component: StorageBoxesEditComponent},
  {path: 'category/:id', component: CategoriesEditComponent},
  {path: 'allocation/:id', component: AllocationsEditComponent},
  {path: 'transaction/:id', component: TransactionsEditComponent},
  {path: 'item-type/:id', component: ItemTypesEditComponent},
];
