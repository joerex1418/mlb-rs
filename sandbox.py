#%%
import mlbapi as mlb
#%%
class Person:
    """
    Paramaters:
    name -> str: person's name
    age -> age: person's age
    """
    def __init__(self,name:str,age:int) -> None:
        self.name: str = name
        self.age: int = age

    # def __annotations__(self):
    #     return "hello"
    

#%%
p = Person("Joe",26)

