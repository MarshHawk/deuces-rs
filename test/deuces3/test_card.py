from deuces3.card_mvp import Card

def test_new():
    assert Card.new("2s") == 69634
    assert Card.new("3h") == 139523
    assert Card.new("4d") == 279045
    assert Card.new("Ac") == 268471337

def test_prime_product_from_rankbits():
    assert Card.prime_product_from_rankbits(0b1000000000001) == 82
    assert Card.prime_product_from_rankbits(0b1000000000010) == 123
    assert Card.prime_product_from_rankbits(0b1000000000100) == 205
    assert Card.prime_product_from_rankbits(0b1000001000000) == 697
    assert Card.prime_product_from_rankbits(7936) == 31367009
    assert Card.prime_product_from_rankbits(124) == 85085
    assert Card.prime_product_from_rankbits(62) == 15015
    assert Card.prime_product_from_rankbits(31) == 2310
    assert Card.prime_product_from_rankbits(4111) == 8610
""" sf = 7936
prime_product = 31367009
sf = 3968
prime_product = 14535931
sf = 1984
prime_product = 6678671
sf = 992
prime_product = 2800733
sf = 496
prime_product = 1062347
sf = 248
prime_product = 323323
sf = 124
prime_product = 85085
sf = 62
prime_product = 15015
sf = 31
prime_product = 2310
sf = 4111
prime_product = 8610 """