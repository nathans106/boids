from tkinter import Canvas, Tk
from database import Database

_NUM_BOIDS = 50
_BOID_SIZE = 10
_CANVAS_SIZE = 500

class Program:
    def __init__(self):
        self._db = Database(_NUM_BOIDS)
        self._root = Tk()
        self._canvas = Canvas(self._root, width=_CANVAS_SIZE, height=_CANVAS_SIZE)
        self._canvas.pack()

        self._boids = {
            id: self._canvas.create_rectangle(0, 0, 0 + _BOID_SIZE, 0 + _BOID_SIZE, fill="red")
            for id in self._db.ids()
        }
        
    def run(self):
        while(True):
            positions = self._db.positions()
            self.draw(positions)
            self._db.update()
        
    def draw(self, positions):
        for (id, (x, y)) in positions.items():
            boid = self._boids[id]
            self._canvas.coords(boid, x, y, x + _BOID_SIZE, y + _BOID_SIZE)
