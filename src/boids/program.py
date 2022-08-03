from tkinter import Canvas, Tk
from database import Database

from boids.gui import Gui

_NUM_BOIDS = 50
_CANVAS_SIZE = 500

class Program:
    def __init__(self):
        self._db = Database(_NUM_BOIDS, _CANVAS_SIZE, _CANVAS_SIZE)
        self._gui = Gui(_CANVAS_SIZE, self._update)

        for id in self._db.ids():
            self._gui.add_boid(id)
        
    def run(self):
        self._gui.start()
        
    def _update(self):
        positions = self._db.positions()
        self._gui.update_boids(positions)
        self._db.update()
