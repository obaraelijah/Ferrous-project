import React from "react";
import { Api } from "../api/api";
import { toast, ToastContentProps } from "react-toastify";

import "../styling/login.css";
import Input from "../components/input";

type LoginProps = {};
type LoginState = {
    username: string;
    password: string;
};

export default class Login extends React.Component<LoginProps, LoginState> {
    constructor(props: LoginProps) {
        super(props);

        this.state = {
            username: "",
            password: "",
        };

        this.performLogin = this.performLogin.bind(this);
    }

    async performLogin(e: React.FormEvent) {
        e.preventDefault();

        (await Api.auth.login(this.state.username, this.state.password)).match(
            async (_: any) => toast.success("Authenticated successfully"),
            (err: { message: string }) => toast.error(err.message)
        );
    }

    render() {
        return (
            <>
                <form onSubmit={this.performLogin}>
                    <div>
                        <label>Username</label>
                        <input
                            type="text"
                            value={this.state.username}
                            onChange={(e) => this.setState({ username: e.target.value })}
                        />
                    </div>
                    <div>
                        <label>Password</label>
                        <input
                            type="password"
                            value={this.state.password}
                            onChange={(e) => this.setState({ password: e.target.value })}
                        />
                    </div>
                    <button type="submit">Login</button>
                </form>
            </>
        );
    }
}