import mlbapi as mlb

def run():
    person: mlb.Person = mlb.get_person(547989)
    
    person.get_stats("season",2021,["hitting"])

if __name__ == "__main__":
    run()

