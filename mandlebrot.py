from copy import deepcopy
from multiprocessing import managers
import numpy as np
from PIL import Image
import matplotlib.pyplot as plt
DIMENSION = 10000
NUM_ITER = 100

def make_image(image_array):
    image_array = deepcopy(image_array+1)
    image_array = image_array/(NUM_ITER+2)
    colormap = plt.get_cmap('magma')
    colors = np.uint8(colormap(image_array)*255)
    image = Image.fromarray(colors)
    with open('mandlebrot.png','wb')as f:
        image.save(f,format='png')



x = np.linspace(-2,2,DIMENSION)
x = np.tile(x,(DIMENSION,1))
y = 1j*np.linspace(-2,2,DIMENSION)
y = np.tile(y,(DIMENSION,1)).T

c = x+y

z = 0j*np.zeros((DIMENSION,DIMENSION))
magnitude = np.abs(c)
image_array = np.zeros((DIMENSION,DIMENSION))
image_array[magnitude<1] = -1
image_array[magnitude>2] = NUM_ITER+1
for i in range(0,NUM_ITER):
    print(i)
    z= z**2 + c
    magnitude = np.abs(z)
    image_array[magnitude<1] = -1
    image_array[magnitude>2] = np.maximum(image_array[magnitude>2],NUM_ITER-i)
    if i%20 == 0:
        make_image(image_array=image_array)
make_image(image_array=image_array)
# magnitude[magnitude<1] = 0.0
# magnitude[magnitude>2] = 2.0

# magnitude= magnitude/2
