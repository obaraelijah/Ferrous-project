import Home from "./views/home";
import { Router } from "./utils/router";
import FerrousNetwork from "./views/ferrous-network";
import Me from "./views/me";
import AdminUsers from "./views/admin/users";
import AdminWorkspaces from "./views/admin/workspaces";
import AdminGuard from "./components/admin-guard";
import Attacks from "./views/attacks";
import AttackResults from "./views/attack-results";
import WorkspaceOverview from "./views/workspace-overview";
import Workspace from "./views/workspace";

export const ROUTER = new Router();

export const ROUTES = {
    HOME: ROUTER.add({ url: "", parser: {}, render: () => <Home /> }),
    ME: ROUTER.add({ url: "me", parser: {}, render: () => <Me /> }),
    WORKSPACES: ROUTER.add({ url: "workspaces", parser: {}, render: () => <WorkspaceOverview /> }),
    WORKSPACE: ROUTER.add({
        url: "workspaces/{uuid}",
        parser: { uuid: String },
        render: ({ uuid }) => <Workspace uuid={uuid} />,
    }),
    ATTACKS: ROUTER.add({ url: "attacks", parser: {}, render: () => <Attacks /> }),

    ATTACK_RESULTS: ROUTER.add({
        url: "attacks/{uuid}",
        parser: { uuid: String },
        render: ({ uuid }) => <AttackResults attackUuid={uuid} />,
    }),
    KNOWLEDGE_BASE: ROUTER.add({ url: "knowledge", parser: {}, render: () => undefined }),
    
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
