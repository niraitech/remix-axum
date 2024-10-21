import { NavLink } from "@remix-run/react";
import { twMerge } from "tailwind-merge";

export const Nav = (): React.ReactElement => {
  return (
    <div className={twMerge("flex", "space-x-4")}>
      <NavLink
        to="/"
        className={({ isActive, isPending }) =>
          isActive ? "text-black" : "text-zinc-300"
        }
      >
        Home
      </NavLink>
      <NavLink
        to="/about"
        className={({ isActive, isPending }) =>
            isActive ? "text-black" : "text-zinc-300"
        }
      >
        About
      </NavLink>
    </div>
  );
};
