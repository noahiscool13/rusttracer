from PIL import Image
import numpy as np

im_a = Image.open("ground_truth_glow.bmp")
im_b = Image.open("vol_rr_glow_400.bmp")

ar_a = np.array(im_a)
ar_b = np.array(im_b)

diff = ar_a-ar_b

square_diff = np.square(diff)

# Image.fromarray(square_diff).show()

print(np.sum(square_diff)/im_a.height/im_b.width)