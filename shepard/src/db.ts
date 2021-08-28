import axios from 'axios';
import { log, Level } from './log';

interface Payload {
    status: String,
    data: any
};

interface SaveRequest {
    key: string,
    name: string,
    pasta: string
}
interface SendRequest {
    key: string,
    name: string
}

interface Headers {
    headers: any
}

export interface DbResponse {
    success: boolean,
    err: Err,
    data?: string
}

export enum Err {
    INVALID_PARENT,
    DUPLICATE_EXISTS,
    DOES_NOT_EXIST,
    UNKNOWN,
    ENV_KEY_UNDETECTED,
    NONE
}

function newDbRes(success: boolean, err: Err, data?: any): DbResponse {
    return {success, err, data}
}

export class DatabaseRequest {
    constructor() {

    }

    //Make the save request to the backend and return the result.
    public static async save(name: string, pasta: string): Promise<DbResponse> {
        //Return if env keys are not detected.
        if (!process.env.auth_key) return newDbRes(false, Err.ENV_KEY_UNDETECTED);
         
        //Set request params
        let url: string = /*'http://localhost:8000/save';*/ 'http://host.docker.internal:8000/save';
        let data: SaveRequest = {
            key: process.env.auth_key,
            name: name,
            pasta: pasta
        };
        let headers: Headers = {
            headers: {
                'Content-Type': 'application/json',
            }
        }
        
        //Try the request
        try {
            let res = await axios.post(url, data, headers);
            let payload: Payload = res.data;
            if (payload.status == 'success') return newDbRes(true, Err.NONE);
            else {
                //Filter though sqlite error messages and set it to an appropriate enum
                switch (true) {
                    case payload.data == "UNIQUE constraint failed: pastas.name":   
                        return newDbRes(false, Err.DUPLICATE_EXISTS);
                        break;
                    default:
                        return newDbRes(false, Err.UNKNOWN);
                }
            }
        } catch (error: Error) {
            log(Level.Error, error.message);
            return newDbRes(false, Err.UNKNOWN)
        }
    }

    //Make the send request to the backend and return the response
    public static async send(name: string): Promise<DbResponse> {
        //Return if env keys are not detected.
        if (!process.env.auth_key) return newDbRes(false, Err.ENV_KEY_UNDETECTED);
         
        //Set request params
        let url: string = /*'http://localhost:8000/send';*/ 'http://host.docker.internal:8000/send';
        let data: SendRequest = {
            key: process.env.auth_key,
            name: name
        };
        let headers: Headers = {
            headers: {
                'Content-Type': 'application/json',
            }
        }
        
        try {
            let res = await axios.post(url, data, headers);
            let payload: Payload = res.data;
            if (payload.status == 'success') return newDbRes(true, Err.NONE, payload.data);
            else {
                //Filter through error messages
                switch (true) {
                    case payload.data == "cannot read a text column":
                        return newDbRes(false, Err.DOES_NOT_EXIST);
                        break;
                    default:
                        return newDbRes(false, Err.UNKNOWN);
                }
            }
        } catch (error: Error) {
            log(Level.Error, error.message);
            return newDbRes(false, Err.UNKNOWN);
        }
    }
}