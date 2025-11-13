import {DatabaseRelation} from './database-relation';

export interface Transaction extends DatabaseRelation {
  allocation_id: number;
  item_delta: number;
  date: Date;
}
