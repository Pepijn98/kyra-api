import Router from "~/api/Router";

export type StatusCodes = 400 | 401 | 403 | 404 | 405 | 406 | 408 | 409 | 410 | 429 | 500 | 501 | 503 | 507 | 505

export type ErrorResponse = {
    statusCode: number
    statusMessage: string
    message: string
}

export type Errors = {
    [T in StatusCodes]: ErrorResponse
}

export type RobotsConfig = {
    userAgent: string | string[]
    disallow: string | string[]
    crawlDelay?: string
    sitemap?: string | string[]
    host?: string | string[]
}

export type Context = {
    path: string
    method: string
    controller: Router
}
