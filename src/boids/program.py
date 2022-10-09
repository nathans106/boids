from simulation import Simulation

from boids.gui import Gui

_NUM_BOIDS = 50
_CANVAS_SIZE = 500

class Program:
    def __init__(self, simulation_parameters_file: str):
        self._simulation = Simulation(_NUM_BOIDS, _CANVAS_SIZE, _CANVAS_SIZE, simulation_parameters_file)
        self._gui = Gui(_CANVAS_SIZE, self._update)

        for id in self._simulation.ids():
            self._gui.add_boid(id)
        
    def run(self):
        self._gui.start()
        
    def _update(self):
        positions = self._simulation.positions()
        self._gui.update_boids(positions)
        self._simulation.advance(1)
