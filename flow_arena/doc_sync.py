import sys, os

"""
doc_sync: writes all the contents in your README file to your `lib.rs` as the lib doc.
By default, it replaces the head of your lib file, with "//!" or space lines, with your README.
The old lines will be printed, in case you accidentally overwrite anything important.
"""

proj_path = os.path.dirname(os.path.abspath(__file__))
readme_path = "README.md"
lib_path = "src/lib.rs"

os.chdir(proj_path)

with open(readme_path, 'r') as f:
    readme = f.readlines()
doc = ["//! " + line for line in readme]

with open(lib_path, 'r') as f:
    lib = f.readlines()

desserted_doc = []

lib = list(reversed(lib))
while len(lib) > 0 and (lib[-1].startswith("//!") or lib[-1].isspace()):
    line = lib.pop()
    desserted_doc.append(line)
lib = list(reversed(lib))

# backup
print(">>>>>>")
for line in desserted_doc:
    print(line, end="")
print("<<<<<<")

lib = doc + ["\n"] + lib


with open(lib_path, 'w') as f:
    f.writelines(lib)
