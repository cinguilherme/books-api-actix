### my second attempt for a microservice using RUST.

#### this project use ACTIX.

##### I have started this mini api just to study purposes.
most of the code i wrote here came from a tutorial on youtube
```https://www.youtube.com/watch?v=e37NbhSm56o```

its a 3 videos series and so far Actix with this set of dependencies have been the best 
experience I had in many languages to build something.


###### this simple micro-service requires a lot of dependencies to be this consise, just like when building a rest api in Node or python.
```
actix-rt = "1.1.1"
actix-web = "2.0.0"
serde = "1.0.111"
dotenv = "0.15.0"
config = "0.10.1"
tokio-pg-mapper = "0.1.8"
tokio-pg-mapper-derive = "0.1.5"
deadpool-postgres = "0.5.5"
tokio-postgres = "0.5.4"
```