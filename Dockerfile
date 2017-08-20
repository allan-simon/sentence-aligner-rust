FROM scratch

ADD target/x86_64-unknown-linux-musl/release/sentence-aligner /  
ENV ROCKET_PORT=80
EXPOSE 80

CMD ["/sentence-aligner"]  

