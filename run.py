import os
from os.path import isdir, join
from shutil import copytree, rmtree

os.system('clear')

# sync = [
#     'static',
#     'tracer.data'
# ]

# des = 'dist'

# for x in sync:
#     dest = join(des, x)
#     if isdir(dest):
#         rmtree(dest)
#     copytree(x, dest)

os.system('trunk serve')
