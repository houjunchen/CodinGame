# https://www.codingame.com/ide/puzzle/hunger-games
class Tribute():
    def __init__(self, name):
        self.name = name
        self.killed = []
        self.killer = None
    def set_killed(self, name):
        self.killed.append(name)
    def set_killer(self, name):
        self.killer = name
    def get_output(self):
        killed = "None" if len(self.killed) == 0 else ", ".join(sorted(self.killed))
        killer = "Winner" if self.killer is None else self.killer
        return "Name: {}\nKilled: {}\nKiller: {}".format(self.name, killed, killer)

tributes = int(input())
players = dict()
for i in range(tributes):
    player_name = input()
    players[player_name] = Tribute(player_name)
turns = int(input())
for i in range(turns):
    info = input()
    s = info.split(" killed ")
    for victim in s[1].split(", "):
        players[s[0]].set_killed(victim)
        players[victim].set_killer(s[0])

print("\n\n".join([players[k].get_output() for k in sorted(players.keys())]))
