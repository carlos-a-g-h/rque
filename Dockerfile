FROM alpine:latest
WORKDIR /app
COPY . .
RUN chmod +x rque
CMD [ "rque" ]
