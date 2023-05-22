import bcrypt from "bcrypt";
import type { Request, Response } from "express";

import Route from "~/api/Route.js";
import Router from "~/api/Router.js";
import { Users } from "~/models/User.js";
import { httpError } from "~/utils/general.js";

type Login = {
    email: string
    password: string
}

export default class extends Route {
    constructor(controller: Router) {
        super({ path: "/auth/login", method: "POST", controller });

        this.controller.router.post(
            this.path,
            this.run.bind(this)
        );
    }

    async run(req: Request, res: Response): Promise<void> {
        try {
            const body: Login = req.body;
            if (!body.email || !body.password) {
                res.status(400).json(httpError[400]);
                return;
            }

            const user = await Users.findOne({ email: body.email }).exec();
            if (!user) {
                const error = { ...httpError[404], message: "No account found with this email address" };
                res.status(404).json(error);
                return;
            }

            const result = await bcrypt.compare(body.password, user.password);
            if (!result) {
                const error = { ...httpError[401], message: "Invalid password" };
                res.status(401).json(error);
                return;
            }

            res.status(200).json({
                statusCode: 200,
                statusMessage: "OK",
                message: "Successfully verified",
                data: {
                    user: user.loginData()
                }
            });
        } catch (error) {
            this.handleException(res, error);
        }
    }
}
