import axios, { AxiosResponse, AxiosRequestConfig } from "axios"
import {HttpPlugin} from "../../index"

jest.mock('axios')

describe('test http plugin get method', () => {
    
    let plugin: HttpPlugin = new HttpPlugin();

    test('valid request: text response type', async() => {
        // @ts-ignore
        axios.get.mockResolvedValueOnce({
            headers: {["Content-Type"]: "application/json; charset=utf-8"},
            status: 200,
            statusText: "Ok",
            data: "{result: 1001}",
            config: {
                responseType: "text"
            }
        } as AxiosResponse)

        const response = await plugin.get("/api/test", {
            headers: [
                {key: "Accept", value: "application/json"}, 
                {key: "X-Test-Header", value: "test-header-value"}
            ],
            urlParams: [
                {key: "q", value: "test-param"}
            ],
            responseType: "TEXT",
        })

        expect(axios.get).lastCalledWith(
            "/api/test", 
            {
                headers: {"Accept": "application/json", "X-Test-Header": "test-header-value"},
                params: {"q": "test-param"},
                responseType: "text",
            } as AxiosRequestConfig,
        )
        
        expect(response.status).toBe(200)
        expect(response.statusText).toBe("Ok")
        expect(response.headers).toStrictEqual([{key: "Content-Type", value: "application/json; charset=utf-8"}])
        expect(response.body).toBe("{result: 1001}")
    })

    test('valid request: arraybuffer response type', async() => {
        // @ts-ignore
        axios.get.mockResolvedValueOnce({
            headers: {["Content-Type"]: "application/json; charset=utf-8"},
            status: 200,
            statusText: "Ok",
            data: Buffer.from("{result: 1001}"),
            config: {
                responseType: "arraybuffer"
            }
        } as AxiosResponse)

        const response = await plugin.get("/api/test", {
            headers: [
                {key: "Accept", value: "application/json"}, 
                {key: "X-Test-Header", value: "test-header-value"}
            ],
            urlParams: [
                {key: "q", value: "test-param"}
            ],
            responseType: "BINARY",
        })

        expect(axios.get).lastCalledWith(
            "/api/test", 
            {
                headers: {"Accept": "application/json", "X-Test-Header": "test-header-value"},
                params: {"q": "test-param"},
                responseType: "arraybuffer",
            } as AxiosRequestConfig,
        )
        
        expect(response.status).toBe(200)
        expect(response.statusText).toBe("Ok")
        expect(response.headers).toStrictEqual([{key: "Content-Type", value: "application/json; charset=utf-8"}])
        expect(atob(response.body)).toBe("{result: 1001}")
    })

    test('failed request', async() => {
        // TODO
    })
})

describe('test http plugin post method', () => {
    
    let plugin: HttpPlugin = new HttpPlugin();

    test('valid request with headers', async() => {
        // @ts-ignore
        axios.post.mockResolvedValueOnce({
            headers: {["Content-Type"]: "application/json; charset=utf-8"},
            status: 200,
            statusText: "Ok",
            data: "{response: 1001}",
            config: {
                responseType: "text"
            }
        } as AxiosResponse)

        const response = await plugin.post("/api/test", {
            headers: [
                {key: "Accept", value: "application/json"}, 
                {key: "X-Test-Header", value: "test-header-value"}
            ],
            urlParams: [
                {key: "q", value: "test-param"}
            ],
            body: "{request: 1001}",
            responseType: "TEXT",
        })

        expect(axios.post).lastCalledWith(
            "/api/test",
            "{request: 1001}", 
            {
                headers: {"Accept": "application/json", "X-Test-Header": "test-header-value"},
                params: {"q": "test-param"},
                responseType: "text",
            } as AxiosRequestConfig,
        )
        
        expect(response.status).toBe(200)
        expect(response.statusText).toBe("Ok")
        expect(response.headers).toStrictEqual([{key: "Content-Type", value: "application/json; charset=utf-8"}])
        expect(response.body).toBe("{response: 1001}")
    })

    test('valid request with url params', async() => {
        // @ts-ignore
        axios.post.mockResolvedValueOnce({
            headers: {["Content-Type"]: "application/json; charset=utf-8"},
            status: 200,
            statusText: "Ok",
            data: Buffer.from("{response: 1001}"),
            config: {
                responseType: "arraybuffer"
            }
        } as AxiosResponse)

        const response = await plugin.post("/api/test", {
            headers: [
                {key: "Accept", value: "application/json"}, 
                {key: "X-Test-Header", value: "test-header-value"}
            ],
            urlParams: [
                {key: "q", value: "test-param"}
            ],
            body: "{request: 1001}",
            responseType: "BINARY",
        })

        expect(axios.post).lastCalledWith(
            "/api/test",
            "{request: 1001}", 
            {
                headers: {"Accept": "application/json", "X-Test-Header": "test-header-value"},
                params: {"q": "test-param"},
                responseType: "arraybuffer",
            } as AxiosRequestConfig,
        )
        
        expect(response.status).toBe(200)
        expect(response.statusText).toBe("Ok")
        expect(response.headers).toStrictEqual([{key: "Content-Type", value: "application/json; charset=utf-8"}])
        expect(atob(response.body)).toBe("{response: 1001}")
    })

    test('failed request', async() => {
        // TODO
    })
})