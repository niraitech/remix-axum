import { NavLink } from "@remix-run/react";
import { twMerge } from "tailwind-merge";

const routes = [
    {"path":"/","name":"Home"},
    {"path":"/about","name":"About"},
    {"path":"/me","name":"Me"}
]

export const Nav = (): React.ReactElement => {
  return (
    <div className={twMerge("flex", "space-x-4")}>
    {routes.map((route) => (
      <NavLink
        key={route.path}
        to={route.path}
        className={({ isActive, isPending }) =>
          isActive ? "text-black" : "text-zinc-300"
        }
      >{route.name}</NavLink>
    ))}
    </div>
  );
};
