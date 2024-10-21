import { json } from "@remix-run/node";
import { useLoaderData } from "@remix-run/react";
import type { CurrentUser } from "../types/server";

export async function clientLoader() {
    const resp = await fetch("/api/users/current_user");
  let data: CurrentUser = await resp.json();
  return data;
}

export default function Me() {
  const { email } = useLoaderData<typeof clientLoader>();
  return (
    <div>
      <h1 className="text-2xl text-cyan-500">Logged in as {email}</h1>
    </div>
  );
}
