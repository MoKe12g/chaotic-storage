import {DatabaseRelation} from './database-relation';

export interface ItemType extends DatabaseRelation {
  storage_property: string;
}
