from tkinter import Canvas, Tk

_BOID_SIZE = 10
_INTERVAL_LENGTH = 50

class Gui:
    def __init__(self, canvas_size, callback):
        self._callback = callback
        self._root = Tk()
        self._canvas = Canvas(self._root, width=canvas_size, height=canvas_size)
        self._boids = {}

        self._canvas.pack()
        self._root.after(_INTERVAL_LENGTH, self._update)
        
    def start(self):
        self._root.mainloop()

    def add_boid(self, id):
        self._boids[id] = self._canvas.create_rectangle(0, 0, 0 + _BOID_SIZE, 0 + _BOID_SIZE, fill="red")

    def update_boid(self, id, pos):
        self._canvas.coords(self._boids[id], pos.x, pos.y, pos.x + _BOID_SIZE, pos.y + _BOID_SIZE)

    def update_boids(self, positions):
        for id, pos in positions.items():
            self.update_boid(id, pos)

    def _update(self):
        self._callback()
        self._root.after(_INTERVAL_LENGTH, self._update)
