import Home from "./views/home";
import { Router } from "./utils/router";
import FerrousNetwork from "./views/ferrous-network";
import Me from "./views/me";
import AdminUsers from "./views/admin/users";
import AdminWorkspaces from "./views/admin/workspaces";
import Workspaces from "./views/workspaces";
import AdminGuard from "./components/admin-guard";

export const ROUTER = new Router();

export const ROUTES = {
    HOME: ROUTER.add({ url: "", parser: {}, render: () => <Home /> }),
    ME: ROUTER.add({ url: "me", parser: {}, render: () => <Me /> }),
    WORKSPACES: ROUTER.add({ url: "workspaces", parser: {}, render: () => <Workspaces /> }),
    
    FERROUS_NETWORK: ROUTER.add({
        url: "ferrous-network",
        parser: {},
        render: () => (
            <AdminGuard>
                <FerrousNetwork />
            </AdminGuard>
        ),
    }),
    ADMIN_USER_MANAGEMENT: ROUTER.add({
        url: "admin/users",
        parser: {},
        render: () => (
            <AdminGuard>
                <AdminUsers />
            </AdminGuard>
        ),
    }),
    ADMIN_WORKSPACE_MANAGEMENT: ROUTER.add({
        url: "admin/workspaces",
        parser: {},
        render: () => (
            <AdminGuard>
                <AdminWorkspaces />
            </AdminGuard>
        ),
    }),
};
ROUTER.finish();
