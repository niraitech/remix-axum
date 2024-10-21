import { Nav } from "./components/nav";
import "./styles/tailwind.css";
import { Links, Meta, Outlet, Scripts } from "@remix-run/react";

export default function App() {
  return (
    <html>
      <head>
        <link rel="icon" href="data:image/x-icon;base64,AA" />
        <Meta />
        <Links />
      </head>
      <body>
        <Nav/>
        <Outlet />

        <Scripts />
      </body>
    </html>
  );
}
