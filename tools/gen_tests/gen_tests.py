def install(package, install_pkg):
  import importlib
  try:
    importlib.import_module(package)
  except ImportError:
    import pip
    pip.main(['install', install_pkg])

def render():
  from mako.template import Template
  import os
  import ujson

  dir_path = os.path.dirname(os.path.realpath(__file__))

  data = None
  with open(os.path.join(dir_path, '..', '..', 'testdata', 'cpu.json'), "r") as f:
    data = ujson.loads(f.read())

  tmpl = Template(filename=os.path.join(dir_path, 'cpu.mako'), module_directory='/tmp/mako_modules')
  print(tmpl.render(data=data), end="")

if __name__ == "__main__":
  # install("mako", "mako")
  # install("ujson", "ujson")
  render()
