import {DatabaseRelation} from './database-relation';

export interface StorageBox extends DatabaseRelation {
  place: string;
  item_type: number;
}
