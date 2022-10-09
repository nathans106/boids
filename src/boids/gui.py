from tkinter import E, N, S, W, Canvas, Entry, Frame, Label, StringVar, Tk
from turtle import width

_BOID_SIZE = 10
_INTERVAL_LENGTH = 50

class Gui:
    def __init__(self, canvas_size, callback):
        self._callback = callback
        self._boids = {}

        self._root = Tk()
        self._root.after(_INTERVAL_LENGTH, self._update)

        self._mainframe = Frame(self._root)
        self._mainframe.grid(column=0, row=0, sticky=(N, W, E, S))

        self._parameters_frame = Frame(self._mainframe)
        self._parameters_frame.grid(column=1, row=0, sticky=(N, E, S))


        # TODO: Populate this with parameter value
        self._fly_to_centre_time = StringVar(value=1)
        self._fly_to_centre_time_entry = Entry(self._parameters_frame, textvariable=self._fly_to_centre_time)
        self._fly_to_centre_time_entry.grid(column=1, row=0)

        self._fly_to_centre_time_name = StringVar(value="Fly to centre time")
        self._fly_to_centre_time_label = Label(self._parameters_frame, textvariable=self._fly_to_centre_time_name)
        self._fly_to_centre_time_label.grid(column=0, row=0)


        #self._fly_to_centre_time_entry.pack()
        #self._fly_to_centre_time_label.pack()

        #self._parameters_frame.pack()
        
        #self._root.pack()

        self._canvas = Canvas(self._mainframe, width=canvas_size, height=canvas_size)
        self._canvas.grid(sticky=(W, N, S))
        #self._canvas.pack()
            
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
