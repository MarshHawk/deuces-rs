cards = ['Ac','Ad','Ah','As','2c','2d','2h','2s','3c','3d','3h','3s','4c','4d','4h','4s','5c','5d','5h','5s','6c','6d','6h','6s','7c','7d','7h','7s','8c','8d','8h','8s','9c','9d','9h','9s','Tc','Td','Th','Ts','Jc','Jd','Jh','Js','Qc','Qd','Qh','Qs','Kc','Kd','Kh','Ks']

def shuffle(cards):
    sample = cards[:]
    #print(sample)
    for (i, x) in enumerate(cards):
        rand = math.floor((i + 1) * random.uniform(0, 1))
        print(rand)
        temp = sample[i]
        print(temp)
        sample[i] = sample[rand]
        print(sample[rand])
        print(sample[i])
        sample[rand] = temp
        print(sample[rand])
    #print(shuff)
    #map(shuff, enumerate(cards))
    return sample