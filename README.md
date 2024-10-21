# Axum and Remix example

A boilerplate repo to integrate remix frontend with an axum backend. 
The goal is to limit Javascript and React's to the frontend and the BFF 
layer alone while being able to share the authentication securely between the
BFF and the backend layers. It uses cognito for auth that Nirai uses. Adapting
it to other OIDC providers should be fairly staight forward.

## Tech choices

Let react and remix focus on rendering and data fetching. Use Rust for everything
else. I use cognito for authentication and stick the ID token in a secure, http-only
cookie. When the page is server rendered, remix is able to read the cookie and 
pass it via the authorization header. I also setup client loaders to skip the hop.
In these cases, the cookie is directly passed to rust which then authenticates it.

This codebase also generates zod types based on your structs. Right now it is
configured via `server/build.rs` but we can build a macro to iterate types or 
simply export it from the `common-types` package.

### Rust and Axum on the backend. 

Rust is a joy to work with and is very compatible
with my brain. Axum makes the job of writing high level HTTP servers
very simple.

### Remix / React / Tailwind for the frontend

Remix is less opinionated than Next and is a surprisingly readable codebase. I
have set it up with tailwind and shadcn.


## :hammer: Configuration

To setup you need to following env vars. Look at `.envrc.example` for more details

```
export COGNITO_CLIENT_ID=<your_client_id>
export COGNITO_CLIENT_SECRET=<your_client_secret>
export COGNITO_DOMAIN=<domain_from_app_integration_page>
export COGNITO_ISSUER_URL=https://cognito-idp.<region>.amazonaws.com/us-east-1_<user_pool_id>
export REDIRECT_URL=https://localhost:5173/auth/callback
```

## :rocket: Running

To run this locally. You need to run the rust server and then the remix app.

```
$ cargo run
```

```
$ pnpm run dev
```

## Tasks

To keep track of tasks in the repo.

* [x] Setup remix project with tailwind
* [x] Setup axum project
* [x] Configure vite to forward requests to axum backend
* [x] Setup loader and client loader to skip the hop
* [x] Setup authentication with cognito
* [x] Setup cookie that can be read by both remix and axum
* [x] Perform token verification via cookie on the axum server
* [x] Perform token verification via header on axum
* [ ] Setup refresh token workflow from cognito
* [ ] Add logout endpoint to clear cookie
* [ ] Logout should also revoke refresh token
* [x] Add a way to automatically generate typescript types from Rust
