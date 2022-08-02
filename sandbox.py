import mlbapi as mlb

def run():
    person: mlb.Person = mlb.rs_get_person(608337)
    print(person.full_name)
    x = person.rs_get_stats("season",2021,["hitting"])
    print(x.stats[0].splits[0].stat.gamesPlayed)

if __name__ == "__main__":
    run()

