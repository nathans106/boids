from tkinter import Canvas, Tk
from boids import Handler

_BOID_SIZE = 10
_CANVAS_SIZE = 500

class Program:
    def __init__(self):
        self._handler = Handler()
        self._root = Tk()
        self._canvas = Canvas(self._root, width=_CANVAS_SIZE, height=_CANVAS_SIZE)
        self._canvas.pack()

        self._boids = {
            id: self._canvas.create_rectangle(0, 0, 0 + _BOID_SIZE, 0 + _BOID_SIZE, fill="red")
            for id in self._handler.get_ids()
        }
        
    def run(self):
        while(True):
            boids = self._handler.get_boids()
            self.draw(boids)
            self._handler.update()
        
    def draw(self, positions):
        for (id, (x, y)) in positions.item():
            boid = self._boids[id]
            self._canvas.coords(boid, x, y, x + _BOID_SIZE, y + _BOID_SIZE)
