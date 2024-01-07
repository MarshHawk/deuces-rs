from deuces3.evaluator import Evaluator
from deuces3.card import Card

def test_evaluate():
    evaluator = Evaluator()
    hand = [Card.new("2s"), Card.new("3h"), Card.new("4d"), Card.new("5c"), Card.new("6s")]
    #assert evaluator.evaluate(hand) == 1600

"""
def test_get_rank():
    evaluator = Evaluator()
    hand = [Card.new("2s"), Card.new("3h"), Card.new("4d"), Card.new("5c"), Card.new("6s")]
    assert evaluator.get_rank(hand) == "Straight"
 """