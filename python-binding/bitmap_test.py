from algo import Bitmap

rb = Bitmap()
rb.insert(2)
rb.insert(3)
rb.insert(5)
rb.insert(7)
assert rb.len() == 4

# is_disjoint
rb1 = Bitmap()
rb2 = Bitmap()
rb1.insert(1)
assert rb1.is_disjoint(rb2) == True
rb2.insert(1)
assert rb1.is_disjoint(rb2) == False

# is_subset
rb1 = Bitmap()
rb2 = Bitmap()
rb1.insert(1)
assert rb1.is_subset(rb2) == False
rb2.insert(1)
assert rb1.is_subset(rb2) == True
rb1.insert(2)
assert rb1.is_subset(rb2) == False

# is_superset
rb1 = Bitmap()
rb2 = Bitmap()
rb1.insert(1)
assert rb2.is_superset(rb1) == False
rb2.insert(1)
assert rb2.is_superset(rb1) == True
rb1.insert(2)
assert rb2.is_superset(rb1) == False


# full
rb = Bitmap.full()
print(rb)

# insert
rb = Bitmap()
assert rb.insert(3) == True
assert rb.insert(3) == False
assert rb.contains(3) == True

# insert_range
rb = Bitmap()
rb.insert_range(2, 4)
assert rb.contains(2)
assert rb.contains(3)
assert not rb.contains(4)

# push
rb = Bitmap()
assert rb.push(1)
assert rb.push(3)
assert not rb.push(3)
assert rb.push(5)
assert rb.get_items() == [1, 3, 5]

# remove
rb = Bitmap()
rb.insert(3)
assert rb.remove(3)
assert not rb.remove(3)
assert not rb.contains(3)

# remove_range
rb = Bitmap()
rb.insert(2)
rb.insert(3)
assert rb.remove_range(2, 4) == 2

# contains
rb = Bitmap()
rb.insert(1)
assert not rb.contains(0)
assert rb.contains(1)
assert not rb.contains(100)

# contains_range
rb = Bitmap()
assert rb.contains_range(7, 7)
rb.insert_range(1, 0xFFF)
assert rb.contains_range(1, 0xFFF)
assert rb.contains_range(2, 0xFFF)
assert not rb.contains_range(0, 2)
assert rb.contains_range(1, 0xFFF)

# range_cardinality
rb = Bitmap()
rb.insert_range(0x10000, 0x40000)
rb.insert(0x50001)
rb.insert(0x50005)
assert rb.range_cardinality(0, 0x10000) == 0
assert rb.range_cardinality(0x10000, 0x40000) == 0x30000
assert rb.range_cardinality(0x50000, 0x60000) == 2
assert rb.range_cardinality(0x10000, 0x10000) == 0

# clear
rb = Bitmap()
rb.insert(1)
assert rb.contains(1)
rb.clear()
assert not rb.contains(1)

# is_empty
rb = Bitmap()
assert rb.is_empty()
rb.insert(1)
assert not rb.is_empty()

# is_full
rb = Bitmap.full()
assert not rb.is_empty()
assert rb.is_full()

# len
rb = Bitmap()
assert rb.len() == 0
rb.insert(3)
assert rb.len() == 1
rb.insert(3)
rb.insert(4)
assert rb.len() == 2

# min
rb = Bitmap()
assert rb.min() == None
rb.insert(3)
rb.insert(4)
assert rb.min() == 3

# max
rb = Bitmap()
assert rb.max() == None
rb.insert(3)
rb.insert(4)
assert rb.max() == 4

# rank
rb = Bitmap()
assert rb.rank(0) == 0
rb.insert(3)
rb.insert(4)
assert rb.rank(3) == 1
assert rb.rank(10) == 2

# select
rb = Bitmap()
assert rb.select(0) == None
rb.append([0, 10, 100])
assert rb.select(0) == 0
assert rb.select(1) == 10
assert rb.select(2) == 100
assert rb.select(3) == None

# remove_smallest
rb = Bitmap.from_iter([1, 5, 7, 9])
rb.remove_smallest(2)
assert rb.get_items() == [7, 9]
rb = Bitmap.from_iter([1, 3, 7, 9])
rb.remove_smallest(2)
assert rb.get_items() == [7, 9]

# remove_biggest
rb = Bitmap.from_iter([1, 5, 7, 9])
rb.remove_biggest(2)
assert rb.get_items() == [1, 5]
rb.remove_biggest(1)
assert rb.get_items() == [1]

# iter
rb = Bitmap.from_iter([1, 5, 7, 9])
it = iter(rb)
assert next(it) == 1
assert next(it) == 5
assert next(it) == 7
assert next(it) == 9

# from_sorted_iter
rb = Bitmap.from_sorted_iter([9, 5, 1, 7])
assert rb.get_items() == [1, 5, 7, 9]

# append
rb = Bitmap()
rb.append([1, 5, 7, 9])
assert rb.get_items() == [1, 5, 7, 9]

# intersection_len
rb1 = Bitmap()
rb1.insert_range(1, 4)
rb2 = Bitmap()
rb2.insert_range(3, 5)
assert rb1.intersection_len(rb2) == 1

# union_len
rb1 = Bitmap()
rb1.insert_range(1, 4)
rb2 = Bitmap()
rb2.insert_range(3, 5)
assert rb1.union_len(rb2) == 4

# difference_len
rb1 = Bitmap()
rb1.insert_range(1, 4)
rb2 = Bitmap()
rb2.insert_range(3, 5)
assert rb1.difference_len(rb2) == 2

# symmetric_difference_len
rb1 = Bitmap()
rb1.insert_range(1, 4)
rb2 = Bitmap()
rb2.insert_range(3, 5)
assert rb1.symmetric_difference_len(rb2) == 3
