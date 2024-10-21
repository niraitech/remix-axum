# Axum and Remix example

A proof-of-concept repo  to integrate remix frontend with an axum backend. 
The goal is to limit Javascript and React's to the frontend and the BFF 
layer alone while being able to share the authentication securely between the
BFF and the backend layers.

* [x] Setup remix project with tailwind
* [x] Setup axum project
* [x] Configure vite to forward requests to axum backend
* [x] Setup loader and client loader to skip the hop
* [x] Setup authentication with cognito
* [x] Setup cookie that can be read by both remix and axum
* [ ] Perform token verification both on remix server 
* [ ] Perform token verification via both cookie and header on axum
* [ ] Setup refresh token workflow from cognito
* [ ] Add logout endpoint to clear cookie
* [ ] Logout should also revoke refresh token
