import Home from "./views/home";
import { Router } from "./utils/router";
import FerrousNetwork from "./views/ferrous-network";
import Me from "./views/me";
import Login from "./views/login";
import AdminUsers from "./views/admin/users";
import AdminWorkspaces from "./views/admin/workspaces";

export const ROUTER = new Router();

export const ROUTES = {
    LOGIN: ROUTER.add({ url: "login", parser: {}, render: () => <Login /> }),
    HOME: ROUTER.add({ url: "", parser: {}, render: () => <Home /> }),
    ME: ROUTER.add({ url: "me", parser: {}, render: () => <Me /> }),
    FERROUS_NETWORK: ROUTER.add({ url: "ferrous-network", parser: {}, render: () => <FerrousNetwork /> }),

    ADMIN_USER_MANAGEMENT: ROUTER.add({ url: "admin/users", parser: {}, render: () => <AdminUsers /> }),
    ADMIN_WORKSPACE_MANAGEMENT: ROUTER.add({ url: "admin/workspaces", parser: {}, render: () => <AdminWorkspaces /> }),
};
ROUTER.finish();
