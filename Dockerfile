FROM rust:latest 
 
RUN apt update && apt upgrade -y 
RUN apt install -y g++-mingw-w64-x86-64
RUN export http_proxy=http://10.140.216.176:3128
RUN export https_proxy=http://10.140.216.176:3128
 
RUN http_proxy=http://10.140.216.176:3128 https_proxy=http://10.140.216.176:3128 rustup target add x86_64-pc-windows-gnu 
RUN http_proxy=http://10.140.216.176:3128 https_proxy=http://10.140.216.176:3128 rustup toolchain install stable-x86_64-pc-windows-gnu 
 
WORKDIR /app 
 
#CMD ["cargo", "build", "--target", "x86_64-pc-windows-gnu"]
CMD ["/bin/bash"]
