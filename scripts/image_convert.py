# %% 
from PIL import Image

# %%
src_path = "../assets/menu/background_0.tiff"
trg_path = "../assets/menu/background_0.png"

# %%
image = Image.open(src_path)
image.save(trg_path)

# %%
