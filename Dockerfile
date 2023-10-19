FROM python:3.13.0a1-slim-bookworm

COPY ./test.py /home
CMD ["python", "/home/test.py"]