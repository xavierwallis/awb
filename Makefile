all:
	docker build -t awb .
	docker run awb
