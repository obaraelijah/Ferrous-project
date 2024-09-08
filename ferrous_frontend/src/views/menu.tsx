import React from "react";
import { ROUTES } from "../routes";
import USER_CONTEXT from "../context/user";
import AttackIcon from "../svg/attack";
import FerrousIcon from "../svg/ferrous";
import KnowledgeIcon from "../svg/knowledge";
import WorkspaceIcon from "../svg/workspace";
import UsersIcon from "../svg/users";
import UserSettingsIcon from "../svg/user_settings";
import "../styling/menu.css";

type MenuItem = "me" | "workspaces" | "attack" | "ferrous" | "users_admin" | "workspaces_admin" | "knowledge";

type MenuProps = {};
type MenuState = {
    active: MenuItem;
};

export default class Menu extends React.Component<MenuProps, MenuState> {
    state: MenuState = {
        active: "workspaces",
    };

    static contextType = USER_CONTEXT;
    declare context: React.ContextType<typeof USER_CONTEXT>;

    render() {
        return (
            <>
                <div className="menu pane">
                <div className={"menu-header"}>
                        <FerrousIcon />
                    </div>
                    <div className={"menu-seperator"}>Workspaces</div>
                    <div className={"menu-item-container"}>
                        <div
                            className={this.state.active === "workspaces" ? "menu-item active" : "menu-item"}
                            onClick={() => {
                                this.setState({ active: "workspaces" });
                                ROUTES.WORKSPACES.visit({});
                            }}
                            onAuxClick={() => {
                                this.setState({ active: "workspaces" });
                                ROUTES.WORKSPACES.open({});
                            }}
                        >
                            <WorkspaceIcon />
                            <div className={"menu-hint"}>Workspaces</div>
                        </div>
                    </div>
                    <div className={"menu-item-container"}>
                        <div
                            className={this.state.active === "attack" ? "menu-item active" : "menu-item"}
                            onClick={() => {
                                this.setState({ active: "attack" });
                                ROUTES.ATTACKS.visit({});
                            }}
                            onAuxClick={() => {
                                this.setState({ active: "attack" });
                                ROUTES.ATTACKS.open({});
                            }}
                        >
                            <AttackIcon />
                            <div className={"menu-hint"}>Attacks</div>
                        </div>
                        </div>
                        <div className={"menu-seperator"}>General</div>
                </div>
                <div className={"menu-item-container"}>
                        <div
                            className={this.state.active === "knowledge" ? "menu-item active" : "menu-item"}
                            onClick={() => {
                                this.setState({ active: "knowledge" });
                                ROUTES.KNOWLEDGE_BASE.visit({});
                            }}
                            onAuxClick={() => {
                                this.setState({ active: "knowledge" });
                                ROUTES.KNOWLEDGE_BASE.open({});
                            }}
                        >
                            <KnowledgeIcon />
                            <div className={"menu-hint"}>Knowledge Base</div>
                    </div>
                </div>
                <div className={"menu-item-container"}>
                        <div
                            className={this.state.active === "me" ? "menu-item active" : "menu-item"}
                            onClick={() => {
                                this.setState({ active: "me" });
                                ROUTES.ME.visit({});
                            }}
                            onAuxClick={() => {
                                this.setState({ active: "me" });
                                ROUTES.ME.open({});
                            }}
                        >
                            <UserSettingsIcon />
                            <div className={"menu-hint"}>User Settings</div>
                        </div>
                    </div>
                    {this.context.user.admin ? (
                        <>
                            <div className={"menu-seperator"}>Admin</div>
                            <div className={"menu-item-container"}>
                                <div
                                    className={this.state.active === "ferrous" ? "menu-item active" : "menu-item"}
                                    onClick={() => {
                                        this.setState({ active: "ferrous" });
                                        ROUTES.FERROUS_NETWORK.visit({});
                                    }}
                                    onAuxClick={() => {
                                        this.setState({ active: "ferrous" });
                                        ROUTES.FERROUS_NETWORK.open({});
                                    }}
                                >
                                    <FerrousIcon />
                                    <div className={"menu-hint"}>Ferrous Network</div>
                                </div>
                            </div>
                            <div className={"menu-item-container"}>
                                <div
                                    className={this.state.active === "users_admin" ? "menu-item active" : "menu-item"}
                                    onClick={() => {
                                        this.setState({ active: "users_admin" });
                                        ROUTES.ADMIN_USER_MANAGEMENT.visit({});
                                    }}
                                    onAuxClick={() => {
                                        this.setState({ active: "users_admin" });
                                        ROUTES.ADMIN_USER_MANAGEMENT.open({});
                                    }}
                                >
                                    <UsersIcon />
                                    <div className={"menu-hint"}>User Controls</div>
                                </div>
                            </div>
                            <div className={"menu-item-container"}>
                                <div
                                    className={
                                        this.state.active === "workspaces_admin" ? "menu-item active" : "menu-item"
                                    }
                                    onClick={() => {
                                        this.setState({ active: "workspaces_admin" });
                                        ROUTES.ADMIN_WORKSPACE_MANAGEMENT.visit({});
                                    }}
                                    onAuxClick={() => {
                                        this.setState({ active: "workspaces_admin" });
                                        ROUTES.ADMIN_WORKSPACE_MANAGEMENT.open({});
                                    }}
                                >
                                    <WorkspaceIcon />
                                    <div className={"menu-hint"}>Workspace Controls</div>
                                    </div>
                            </div>
                    </>
                    ) : null}

                <div className={"workspace-selector-container pane"}>
                    <WorkspaceIcon />
                </div>
            </>
        );
    }
}