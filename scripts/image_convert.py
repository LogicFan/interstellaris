# %% 
from PIL import Image

src_path = "../assets/background/background_0.tif"
trg_path = "../assets/background/background_small_0.png"

image = Image.open(src_path)
image = image.resize((1920, 1080))
image.save(trg_path)

# %%
