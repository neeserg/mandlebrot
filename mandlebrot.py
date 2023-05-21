from copy import deepcopy
import numpy as np
from PIL import Image
import matplotlib.pyplot as plt
from datetime import datetime
DIMENSION = 10000
NUM_ITER = 100
upper_imaginary = 2
lower_imgaginary = -2
lower_real = -2
upper_real = 2.0
def make_image(image_array):
    image_array = deepcopy(image_array+1)
    image_array = image_array/(NUM_ITER+2)
    colormap = plt.get_cmap('magma')
    colors = np.uint8(colormap(image_array)*255)
    image = Image.fromarray(colors)
    with open('mandlebrot.png','wb')as f:
        image.save(f,format='png')


def pure_pythonic_mandlebrot():
    ##has a bit of numpy for ease but mostly python for looping.
    image_array = np.zeros((DIMENSION,DIMENSION))
    image_array = image_array+NUM_ITER
    x = np.linspace(lower_real,upper_real,DIMENSION)
    x = np.tile(x,(DIMENSION,1))
    y = 1j*np.linspace(lower_imgaginary,upper_imaginary,DIMENSION)
    y = np.tile(y,(DIMENSION,1)).T
    c = x+y

    for i in range(DIMENSION):
        for j in range(DIMENSION):
            complex_const = c[i][j]
            z_k = 0j+0
            for k in range(NUM_ITER):
                z_k = z_k*z_k +complex_const
                if np.abs(z_k)>2:
                    image_array[i][j] = k
                    break

    make_image(image_array=image_array)

def make_mandlebrot():

    x = np.linspace(lower_real,upper_real,DIMENSION)
    x = np.tile(x,(DIMENSION,1))
    y = 1j*np.linspace(lower_imgaginary,upper_imaginary,DIMENSION)
    y = np.tile(y,(DIMENSION,1)).T

    c = x+y

    z = 0j*np.zeros((DIMENSION,DIMENSION))
    magnitude = np.abs(c)
    image_array = np.zeros((DIMENSION,DIMENSION))
    image_array = image_array+NUM_ITER
    for i in range(0,NUM_ITER):
        z= z**2 + c
        magnitude = np.abs(z)
        image_array[magnitude>2] = np.minimum(image_array[magnitude>2],i)
    make_image(image_array=image_array)

before = datetime.now()

NUM_OF_TRIALS = 5
for i in range(NUM_OF_TRIALS):
    trial_before = datetime.now()
    make_mandlebrot()
    # pure_pythonic_mandlebrot()
    total_seconds = (datetime.now()-trial_before).total_seconds()
    print(f'Trial number {i} finishes in: {total_seconds//60} minutes {total_seconds%60} seconds')

after_seconds = (datetime.now() - before).total_seconds()
average_seconds = after_seconds/NUM_OF_TRIALS
print(f'Total time elapsed is: {after_seconds//60} minutes and {after_seconds%60} seconds')
print(f'Average seconds is: {average_seconds//60} minutes and {average_seconds%60} seconds')
