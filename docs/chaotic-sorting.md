# Chaotic-Sorting
Storage system that keeps track of items and where they are stored.

The name could be a bit more work, but not yet.

## SQL Database
### storage_boxes
Boxes or places that can be used for storing things. Like a euro box, a euro-pallet or a compartment in a cabinet.
They get an id, so they can be found easily.
### categories
Everything stored can be assigned a category
### allocations
When using a storage_box to store something you create a allocation. An allocation means, that this part can be found inside of the storage_box that it has the primary key of.
An allocation only means, that this thing can be there, it doesn't mean that there is one or only one there. How many of the specific thing are inside of the storage_box can be calculated via the transactions.
Its more like a reservation an item. There could be 0, 1 or n items there. An item could also have multiple storage_boxes (multiple allocation records)
### transactions
When an item gets taken out or added to an allocation one record gets created with the delta (the change of the number of items). When every entry with a allocation_id get sumed up, you've got the number of items stored right now.
### type
Specifies the physical properties of the item. Like bulky goods, small parts, store in closed containers
