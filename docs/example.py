from enum import Enum
import random
from re import T
from typing import List
from termcolor import colored
class Colors(Enum):
    BLACK = 0
    RED = 1
    GREEN = 2
    
class Roulette(object):
    def __init__(self):
        self.colors: List[str] = []
        self.__populate()
        
    def __populate(self):
        for _ in range(0, 18):
            self.colors.append(Colors.BLACK)
            self.colors.append(Colors.RED)
        
        self.colors.append(Colors.GREEN)
        
    def print_colors(self):
        for color in self.colors:
            if color == Colors.RED:
                print(colored("#", 'red'), end='')
            elif color == Colors.GREEN:
                print(colored("#", 'green'))
            else:
                print("#", end='')
    
    def rotate(self):
        return random.choice(self.colors)
    
class Player(object):
    def __init__(self, roulette: Roulette = Roulette(), max_loss_streak: int = 5, minimum_bet: int = 1):
        self.roulette: Roulette = roulette
        self.win_amount: float = 0.0
        self.loss_amount: float = 0.0
        self.loss_streak: int = 0
        self.win_counter: int = 0
        self.loss_counter: int = 0
        self.max_loss_streak: int = max_loss_streak
        self.max_loss_counter: int = 0
        self.minimum_bet: int = minimum_bet
        
    def bet(self, color, value = None, statistics = False):   
        if value is None:
            value = self.loss_streak * self.minimum_bet * 2 if self.loss_streak > 0 else self.minimum_bet         
        
        cashback = 0.5/100*value
        self.win_amount += cashback
        
        print(f"Betting R${value} on color: {color}")
        print(f"Cashback: {cashback}")
        
        result = self.roulette.rotate()
        
        if result == color:
            
            self.win_counter += 1
            
            if self.loss_streak > 0:
                self.loss_streak = 0
                self.win_amount += self.minimum_bet
            else:
                self.win_amount += 2*value
            print(colored("WIN", "green"))
        else:
            self.loss_counter += 1
            self.loss_amount -= value
            self.loss_streak += 1
            
            if self.loss_streak == self.max_loss_streak:
                print(colored("################# REACHED MAX LOSS STREAK #################", "red"))
                self.max_loss_counter += 1
                self.loss_streak = 0
                
            print(colored("LOSE", "red"))
        
        
        
        if statistics:
            self.show_statistics()    
        
    def show_statistics(self, csv=False, to_file=False):
        if csv:
            line = f"{self.win_amount}, {self.loss_amount}, {self.win_counter}, {self.loss_counter}\n"
            if to_file:
                with open("results.csv", "a+") as f:
                    f.write(line)
            print(line)
            return
        
        print("#" * 40)
        print("Win R$: ", self.win_amount)
        print("Loss R$: ", self.loss_amount)
        #print(f"Reached max loss {self.max_loss_counter} times")
        
        print(f"Win/Loss ratio: {self.win_counter}/{self.loss_counter}")
        
        print("Profit: ", self.win_amount + self.loss_amount)
        print("#" * 40)
    
        
        
def main():
    bets = 10000000
    player = Player(max_loss_streak=5, minimum_bet=1)
    player.roulette.print_colors()
    
    while bets > 0:
        
        color = Colors.RED if bets % 2 == 0 else Colors.BLACK
        player.bet(color=color, statistics=False)
        
        bets -= 1
        
    player.show_statistics(csv=False, to_file=False)
    
if __name__ == '__main__':
    main()