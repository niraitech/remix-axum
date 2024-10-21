import { json, LoaderFunctionArgs } from "@remix-run/node";
import { useLoaderData } from "@remix-run/react";
import type { CurrentUser } from "../types/server";

export async function loader({request}: LoaderFunctionArgs) {
    const cookieHeader = request.headers.get("Cookie");
  if (!cookieHeader) {
    throw new Response("Unauthorized", { status: 401 });
  }

  // Parse the cookie to get the auth_token (assuming it was set as 'auth_token')
  const cookies = Object.fromEntries(cookieHeader.split('; ').map(c => c.split('=')));
  const authToken = cookies["auth_token"];
  
  if (!authToken) {
    throw new Response("Unauthorized", { status: 401 });
  }

    const resp = await fetch("http://localhost:4444/api/users/current_user", {
        headers: {
            "Authorization": authToken
        }
    });
    console.log(resp);
  let data: CurrentUser = await resp.json();
  return json(data);
}

export async function clientLoader() {
    const resp = await fetch("/api/users/current_user");
  let data: CurrentUser = await resp.json();
  return data;
}

export function HydrateFallback() {
  return <p>Loading User...</p>;
}

export default function Me() {
  const { email } = useLoaderData<typeof clientLoader>();
  return (
    <div>
      <h1 className="text-2xl text-cyan-500">Logged in as {email}</h1>
    </div>
  );
}
