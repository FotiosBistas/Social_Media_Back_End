FROM debian:latest 
#I looked at the docker website and saw that I need debian latest
#lookup anatomy of a docker file 
RUN mkdir -p /client/src

WORKDIR /client/src

#manage dependencies with json if we had one 
#COPY package.json .
#if we needed npm to install our packages 
#RUN npm install 

#if copy <src> <dest> src is left empty then it copies the curr dir
COPY . client/src 


CMD ["cargo","run"]