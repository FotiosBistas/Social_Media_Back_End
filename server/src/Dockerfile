FROM debian:latest 
#I looked at the docker website and saw that I need debian latest
#lookup anatomy of a docker file 
RUN mkdir -p /server/src

WORKDIR /server/src

#manage dependencies with json if we had one 
#COPY package.json .
#if we needed npm to install our packages 
#RUN npm install 

COPY . server/src

#the server uses the port 8080 to listen to connections 
EXPOSE 8080 

CMD ["npm","start"]