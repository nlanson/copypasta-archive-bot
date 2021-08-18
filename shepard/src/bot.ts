//Dependencies
import Snoowrap, { SnoowrapOptions, Comment } from 'snoowrap';
import { CommentStream } from 'snoostorm';
import axios from 'axios';
import { log, Level } from './log';


interface Payload {
    status: String,
    data: any
};

interface DbResponse {
    success: boolean,
    err: Err,
    data?: string
}

enum Err {
    INVALID_PARENT,
    DUPLICATE_EXISTS,
    DOES_NOT_EXIST,
    UNKNOWN,
    NONE
}

function newDbRes(success: boolean, err: Err, data?: any): DbResponse {
    return {success, err, data}
}

export class Bot {
    private reddit: Snoowrap;
    private connectedAt: number;

    constructor() {
        try {
        this.reddit = new Snoowrap(this.loadBotCredentials());
        } catch (err: any) {
            log(Level.Error, `Failed to connect due to '${err.message}' Exiting...`);
            process.exit(1);
        }
        log(Level.Info, "Bot connected.");
        this.connectedAt = Date.now() / 1000;
        this.streamComments('u_keijyu'); //Set subreddit to stream comments
    }

    private loadBotCredentials(): SnoowrapOptions {
        let credentials: SnoowrapOptions;
        if (
            process.env.bot_username &&
            process.env.bot_password &&
            process.env.bot_userAgent &&
            process.env.bot_clientId &&
            process.env.bot_clientSecret
        ) {
            credentials = {
                username: process.env.bot_username,
                password: process.env.bot_password,
                userAgent: process.env.bot_userAgent,
                clientId: process.env.bot_clientId,
                clientSecret: process.env.bot_clientSecret
            };

            return credentials;
        } 
        else {
            log(Level.Error, "Environment credentials not detected. Exiting...");
            process.exit(1);
        }
    }

    private streamComments(subreddit: string) {
        const comments: CommentStream = new CommentStream(this.reddit, {
            subreddit: subreddit,
            pollTime: 1500
        });
        log(Level.Info, `Subscribed to ${subreddit}`);

        comments.on('item', async (comment: Comment) => {
            //Extract commend into variable and split into args
            let content: String = comment.body.toLowerCase();
            let args: Array<String> = content.split(" ");

            //Stop processing if comment is older than bot start date or doesn't start with keyword
            if ( this.connectedAt > comment.created_utc ) return;
            if (args[0] != '!save' && args[0] != '!send') return; 
            let key: string = args.slice(1, args.length).join(" ");

            //Run through respective pipe process.
            if (args[0] == '!save') {

                log(Level.Info, `Save requested by u/${comment.author.name}`);
                let saveStatus = await this.savePastaToDb(comment.parent_id, key);
                if (saveStatus.success) await this.reply2thread(comment, "Saved. Use the command " + "` !send "+ `${key}` + " ` to paste");
                else {
                    log(Level.Error, "Save failed");
                    let msg: string;
                    switch (+saveStatus.err) {
                        //Filter through error reasons and set appropriate reply message.
                        case Err.DUPLICATE_EXISTS:
                            msg = "Failed to save. Another pasta with the name " + "` " + key + " ` already exists.";
                            break;
                        case Err.INVALID_PARENT:
                            msg = "The text you tried to save is invalid.\nPlease retry with a post/comment with text only.";
                            break;
                        default:
                            msg = "Unexpected error occured when saving.\nPlease contact u/keijyu if this issue persists.";
                    }
                    await this.reply2thread(comment, msg);
                }
                return;

            } else if (args[0] == '!send') {

                log(Level.Info, `Send requested by u/${comment.author.name}`);
                let getStatus = await this.getPastaFromDb(key);
                if (getStatus.success && getStatus.data) await this.reply2thread(comment, getStatus.data);
                else {
                    log(Level.Error, "Pasta retrieval failed");
                    let msg: string;
                    switch (+getStatus.err) {
                        case Err.DOES_NOT_EXIST:
                            msg = `The pasta you ordered does not exist. Try order some pizza instead.`;
                            break;
                        default:
                            msg = "Unexpected error occured when saving.\nPlease contact u/keijyu if this issue persists.";
                    }
                    await this.reply2thread(comment, msg);
                }
                return;

            } else {
                log(Level.Warning, "Invalid command");
                return;
            }
        });
    }

    //Function that extracts the parent comment and sends the save request to the database.
    private async savePastaToDb(parent: string, name: String): Promise<DbResponse> {
        let pasta: string;
        switch (true) {
            case parent[1] == "1":
                pasta = await this.reddit.getComment(parent).body;
                break;
            case parent[1] == "3":
                pasta = await this.reddit.getSubmission(parent).selftext;
                break;
            default:
                log(Level.Warning, "Invalid parent. Aborting...");
                return newDbRes(false, Err.INVALID_PARENT);
        }

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

    //Function that fetches the reeusted pasta from the database.
    private async getPastaFromDb(name: String): Promise<DbResponse> {
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

    private async reply2thread(comment: Comment, msg: string): Promise<boolean> {
        await comment.reply(msg)
        .then(() => {
            return true;
        })
        .catch((err) => {
            log(Level.Error, err.message);
            return false;
        })
        .finally(() => {
            return false;
        });
        return false;
    }
}