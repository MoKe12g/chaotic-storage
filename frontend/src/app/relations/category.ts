import {DatabaseRelation} from './database-relation';

export interface Category extends DatabaseRelation {
  comment: string;
}
