# Python solutions for advent of code
These are my solutions to advent of code using python. I'm also trying to use
this as a project to practice python project setup etc

## Setup
Setup local python environment by running
```sh
# Install pyenv, requires updating shell config
curl -fsSL https://pyenv.run | bash

# Install expected python version
pyenv install 3.13
pyenv local 3.13

# Setup project local environment
python -m venv .venv
source .venv/bin/activate

# Install all required libraires
pip install -r requirements.txt
```

## Usage
If you set up using `pyenv-virtualenv` your shell can automagically load your
local environment just by entering the directory. Otherwise, use the steps
below.
```sh
source .venv/bin/activate
python <year>/<day>.py
```
