install:
	pip install --upgrade pip &&\
		pip install -r requirements.txt

test:
	python -m pytest -vv --cov=main  test_*.py

format:	
	black *.py 

lint:
	#disable comment to test speed
	#pylint --disable=R,C --ignore-patterns=test_.*?py *.py mylib/*.py
	#ruff linting is 10-100X faster than pylint
	ruff check  *.py *.py

container-lint:
	docker run --rm -i hadolint/hadolint < Dockerfile

refactor: format lint

run:
# 	python main.py
		
all: install lint test format refactor