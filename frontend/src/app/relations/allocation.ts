import {DatabaseRelation} from './database-relation';

export interface Allocation extends DatabaseRelation {
  description: string;
  date_of_entry: Date;
  can_be_outside?: boolean;
  category_id: number;
  storage_box_id: number;
}
