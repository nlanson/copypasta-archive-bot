import axios from 'axios';
import { log, Level } from './log';

interface Payload {
    status: String,
    data: any
};

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
        try {
            let res = await axios
                .get(`http://localhost:8000/save/${process.env.auth_key}/${name}/${pasta}`);
                //.get(`http://host.docker.internal:8000/save/${process.env.auth_key}/${name}/${pasta}`);
            let payload: Payload = res.data;
            if (payload.status == 'success') return newDbRes(true, Err.NONE);
            else {
                //Filter though error messages
                switch (true) {
                    case payload.data == "UNIQUE constraint failed: pastas.name":   
                        return newDbRes(false, Err.DUPLICATE_EXISTS);
                        break;
                    default:
                        return newDbRes(false, Err.UNKNOWN);
                }
            }
        } catch (error) {
            log(Level.Error, error.message);
            return newDbRes(false, Err.UNKNOWN)
        }
    }

    //Make the send request to the backend and return the response
    public static async send(name: string): Promise<DbResponse> {
        try {
            let res = await axios
                .get(`http://localhost:8000/send/${process.env.auth_key}/${name}`);
                //.get(`http://host.docker.internal:8000/send/${process.env.auth_key}/${name}`);
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
        } catch (error) {
            log(Level.Error, error.message);
            return newDbRes(false, Err.UNKNOWN);
        }
    }
}