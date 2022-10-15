import imp
from multiprocessing import managers
from opcode import opname


import numpy as np
from PIL import Image
import matplotlib.pyplot as plt
DIMENSION = 10000
x = np.linspace(-2,2,DIMENSION)
x = np.tile(x,(DIMENSION,1))
y = 1j*np.linspace(-2,2,DIMENSION)
y = np.tile(y,(DIMENSION,1)).T

c = x+y

z = np.zeros((DIMENSION,DIMENSION))

for i in range(100):
    z = z**2 + c

magnitude = np.abs(z)

magnitude[magnitude<1] = 0.0
magnitude[magnitude>2] = 2.0

magnitude= magnitude/2

colormap = plt.get_cmap('magma')

colors = np.uint8(colormap(magnitude)*255)
image = Image.fromarray(colors)
image.save(open('mandlebrot.png','wb'),format='png')