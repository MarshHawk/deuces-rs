from deuces3.lookup import LookupTable
from deuces3.lexograph import generate_next_permutation

def test_init():
    lookup_table = LookupTable()
    flushes = lookup_table.flush_lookup
    unsuited = lookup_table.unsuited_lookup
    # [(31367009, 1), (14535931, 2), (6678671, 3), (2800733, 4)]
    #print([(k, v) for k, v in flushes.items()][0:4])
    assert len(flushes) == 1287
    assert all(isinstance(flush, int) for flush in flushes)
    assert len(unsuited) == 6175
    assert all(isinstance(unsuited, int) for unsuited in unsuited)

def test_get_lexographically_next_bit_sequence():
    lookup_table = LookupTable()
    f_int = int('0b11111', 2)
    bit_gen = lookup_table.get_lexographically_next_bit_sequence(f_int)
    assert next(bit_gen) == 47
    assert next(bit_gen) == 55
    assert next(bit_gen) == 59
    #print(next(bit_sequence_generator))
    #assert next(bit_sequence_generator) == 0b1100
    #assert next(bit_sequence_generator) == 0b1110
    #assert next(bit_sequence_generator) == 0b1111

""" def test_straight_and_highcards():
    lookup_table = LookupTable()
    straights, highcards = lookup_table.straight_and_highcards()
    assert len(straights) == 10
    assert all(isinstance(straight, int) for straight in straights)
    assert len(highcards) == 1277
    assert all(isinstance(highcard, int) for highcard in highcards)

def test_multiples():
    lookup_table = LookupTable()
    multiples = lookup_table.multiples()
    assert len(multiples) == 7462
    assert all(isinstance(multiple, int) for multiple in multiples)

def test_write_table_to_disk(tmp_path):
    lookup_table = LookupTable()
    table = [1, 2, 3, 4, 5]
    filepath = tmp_path / "lookup_table.txt"
    lookup_table.write_table_to_disk(table, filepath)
    assert filepath.exists()

def test_get_lexographically_next_bit_sequence():
    lookup_table = LookupTable()
    bit_sequence_generator = lookup_table.get_lexographically_next_bit_sequence(0b1010)
    assert next(bit_sequence_generator) == 0b1100
    assert next(bit_sequence_generator) == 0b1110
    assert next(bit_sequence_generator) == 0b1111 """

""" def test_flushes():
    lookup_table = LookupTable()
    flushes = lookup_table.flushes()

    # Test the length of flushes
    assert len(flushes) == 1277

    # Test the type of each element in flushes
    assert all(isinstance(flush, int) for flush in flushes)

    # Test that straight flushes are not included in flushes
    straight_flushes = [
        7936, 3968, 1984, 992, 496, 248, 124, 62, 31, 4111
    ]
    assert all(flush not in straight_flushes for flush in flushes)

    # Test the ranking of flushes
    rank = LookupTable.MAX_FULL_HOUSE + 1
    for f in flushes:
        prime_product = Card.prime_product_from_rankbits(f)
        assert lookup_table.flush_lookup[prime_product] == rank
        rank += 1 """