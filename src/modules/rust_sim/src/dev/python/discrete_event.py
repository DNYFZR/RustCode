# Discrete Event
import numpy as np

class DiscreteEvent:

    def __init__(self, states, probs):
        self.states = states
        self.probs = probs
        self.state_matrix = np.zeros([1])
        self.event_matrix = np.zeros([1])

    def configure(self, n_timesteps=50, n_sims=1_000):
        start_state = np.array(self.states, dtype=np.uint8).reshape((len(self.states), 1))
        future_states = np.empty((len(self.states), n_timesteps), dtype=np.uint8)
        iteration = np.concatenate((start_state, future_states), axis=1).astype(np.uint8)
        self.state_matrix = np.stack([iteration for _ in range(n_sims)], axis=0)
        return self

    def run(self, trace=False):            
        for i in range(self.state_matrix.shape[2]-1):
            if trace==True and i % 10==0:
                print(i)
            timestep = self.state_matrix[:, :, i].flatten()
            timestep = np.where(self.probs[timestep] > np.random.rand(len(timestep)), timestep + 1, 0)
            timestep = timestep.reshape((self.state_matrix.shape[0], self.state_matrix.shape[1]))
            self.state_matrix[:, :, i+1] = timestep
        return self

    def convert(self):
        self.event_matrix = np.where(self.state_matrix == 0, 1, 0)
        return self
      
if __name__ == '__main__':
    from time import time
    ages = np.random.randint(0, 5, size=90_000, dtype=np.uint8)
    probs = np.array([1., 0.9, 0.8, 0.7, 0.6, 0.5, 0.])

    start = time()

    sim = DiscreteEvent(states=ages, probs=probs)
    
    raw = sim.configure()
    run = raw.run()
    interval = time()

    conv = run.convert()
    end = time()

    print(f'''
        run : {interval - start}
        convert : {end - interval}
        ''')
