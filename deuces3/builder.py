import math
import random
from deuces3.evaluator import Evaluator
from deuces3.card import Card
from proto.hand_pb2 import (
    Hand,
    Board,
    HandResponse,
)

cards = ['Ac', 'Ad', 'Ah', 'As', '2c', '2d', '2h', '2s', '3c', '3d', '3h', '3s', '4c', '4d', '4h', '4s', '5c', '5d', '5h', '5s', '6c', '6d', '6h', '6s', '7c',
         '7d', '7h', '7s', '8c', '8d', '8h', '8s', '9c', '9d', '9h', '9s', 'Tc', 'Td', 'Th', 'Ts', 'Jc', 'Jd', 'Jh', 'Js', 'Qc', 'Qd', 'Qh', 'Qs', 'Kc', 'Kd', 'Kh', 'Ks']


def shuffle():
    sample = cards[:]
    for (i, x) in enumerate(cards):
        rand = math.floor((i + 1) * random.uniform(0, 1))
        temp = sample[i]
        sample[i] = sample[rand]
        sample[rand] = temp
    return sample


def gen_next_index():
    index = 0
    yield index
    while (True):
        index += 1
        yield index


def build_deal_payload(playerCount):
    evaluator = Evaluator()
    cards = shuffle()
    hands = []
    nextn = gen_next_index()
    i = 0
    players = range(0, playerCount)
    pl = len(players)
    for p in players:
        i = next(nextn)
        player_hand = [cards[i], cards[i+len(players)]]
        score = [Card.new(card) for card in player_hand]
        hands.append({"hand": player_hand, "scores": score})

    flop = [cards[next(nextn)+pl], cards[next(nextn)+pl],
            cards[next(nextn)+pl]]
    flop_score = [Card.new(card) for card in flop]
    turn = cards[next(nextn)+pl]
    turn_score = Card.new(turn)
    river = cards[next(nextn)+pl]
    river_score = Card.new(river)
    phs = []
    for hand in hands:
        hand_score = evaluator.evaluate(
            flop_score + [turn_score] + [river_score], hand["scores"])
        score = evaluator.get_rank_class(hand_score)
        percentage = 1.0 - evaluator.get_five_card_rank_percentage(hand_score)
        description = evaluator.class_to_string(score)
        phs.append(
            Hand(cards=hand["hand"], score=percentage, description=description))
    board = Board(flop=flop, turn=turn, river=river)
    return HandResponse(board=board, hands=phs)
