import { json } from "@remix-run/node";
import { useLoaderData } from "@remix-run/react";
import type { Message } from "../types/server";

export async function loader() {
  const resp = await fetch("http://localhost:4444/api/about");
  let data: Message = await resp.json();
  return json(data);
}

export async function clientLoader() {
  const resp = await fetch("/api/about");
  let data: Message = await resp.json();
  return data;
}

export default function About() {
  const { message } = useLoaderData<typeof loader>();
  return (
    <div>
      <h1 className="text-2xl text-cyan-500">{message}</h1>
    </div>
  );
}
