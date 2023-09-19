FROM alpine:latest
WORKDIR /rque.server
COPY . .
RUN chmod +x rque
CMD [ "./rque" ]
