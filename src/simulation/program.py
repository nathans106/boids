from boids import Handler

class Program:
    def __init__(self):
        self.handler = Handler()

    def run(self):
        while(True):
            boids = self.handler.get_boids()
            self.draw(boids)
            self.handler.update()
        
    def draw(self, boids):
        raise NotImplementedError
