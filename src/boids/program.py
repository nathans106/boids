from tkinter import E, N, W, S, Frame, Tk
from simulation import Simulation

from boids.graphic.sky import Sky

class Program:
    def __init__(self, parameters_file: str):
        self._root = Tk()

        self._mainframe = Frame(self._root)
        self._mainframe.grid(column=0, row=0, sticky=(N, W, E, S))

        self._sky = Sky(self._mainframe, parameters_file)
        self._sky.grid(sticky=(W, N, S))

        
    def run(self):
        self._root.mainloop()
