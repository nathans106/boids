from simulation import Simulation

from tkinter import Canvas


_BOID_SIZE = 10
_INTERVAL_LENGTH = 50
_NUM_BOIDS = 50
_CANVAS_SIZE = 500

class Sky(Canvas):
    def __init__(self, master, parameters_file):
        super().__init__(master, width=_CANVAS_SIZE, height=_CANVAS_SIZE)
        self.__simulation = Simulation(_NUM_BOIDS, _CANVAS_SIZE, _CANVAS_SIZE, parameters_file)
        self.__boids = {}

        for id in self.__simulation.ids():
            self.__boids[id] = self.create_rectangle(0, 0, 0 + _BOID_SIZE, 0 + _BOID_SIZE, fill="red")

        self.__refresh()
        self.after(_INTERVAL_LENGTH, self.__update)

    def __refresh(self):
        positions = self.__simulation.positions()
        for id, pos in positions.items():
            self.coords(self.__boids[id], pos.x, pos.y, pos.x + _BOID_SIZE, pos.y + _BOID_SIZE)

    def __update(self):
        self.__simulation.advance(1)
        self.__refresh()
        self.after(_INTERVAL_LENGTH, self.__update)
