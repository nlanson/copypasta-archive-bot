//Dependencies
import Snoowrap, { SnoowrapOptions, Comment } from 'snoowrap';
import { CommentStream } from 'snoostorm';
import axios from 'axios';
import { log, Level } from './log';

interface Payload {
    status: String,
    data: any
};

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

            //Process save and send commands seperately
            if (args[0] == '!save') {
                //On save command
                log(Level.Info, `Save requested by u/${comment.author.name}`);

                let key: string = args.slice(1, args.length).join(" ");
                let saved = await this.savePasta(comment.parent_id, key);
                if (saved) {
                    this.reply2thread(comment, "Saved. Use the command " + "` !send "+ `${key}` + " ` to paste");
                } else {
                    log(Level.Error, "Save failed");
                    this.reply2thread(comment, "Failed to save. Another pasta with the name " + "` " + key + " ` already exists.");
                }
                return;

            } else if (args[0] == '!send') {
                //On send command
                log(Level.Info, `Send requested by u/${comment.author.name}`);

                let key: string = args.slice(1, args.length).join(" ");
                let pasta = await this.getPasta(key);
                if (pasta.length != 0) {
                    //If the pasta exists
                    this.reply2thread(comment, pasta);
                } else {
                    //If the pasta doesn't exist
                    log(Level.Error, "Pasta retrieval failed");
                    this.reply2thread(comment, `The pasta you ordered does not exist. Try order some pizza instead.`)
                }

                return;
            } else {
                //On invalid command
                log(Level.Warning, "Invalid command");
                return;

            }
        });
    }

    //Function that extracts the parent comment and sends the save request to the database.
    private async savePasta(parent: string, name: String): Promise<Boolean> {
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
                return false;
        }

        //Change localhost to the Server IP for production.
        let success: Boolean = false;
        await axios
            //.get(`http://host.docker.internal:8000/save/${process.env.auth_key}/${name}/${pasta}`)
            .get(`http://localhost:8000/save/${process.env.auth_key}/${name}/${pasta}`)
            .then((res: any) => {
                let payload: Payload = res.data;
                if (payload.status == 'success') success = true;
                else {
                    success = false;
                }
            })
            .catch((error: any) => {
                log(Level.Error, error.message);
                success = false;
            });
        return success;
    }

    //Function that fetches the reeusted pasta from the database.
    private async getPasta(name: String): Promise<string> {
        let pasta: string = '';

        await axios
            //.get(`http://host.docker.internal:8000/send/${process.env.auth_key}/${name}`)
            .get(`http://localhost:8000/send/${process.env.auth_key}/${name}`)
            .then((res: any) => {
                let payload: Payload = res.data;
                if (payload.status == 'success') {
                    pasta = payload.data;
                }
            })
            .catch((error: any) => {
                log(Level.Error, error.message);
            });
        
        return pasta;
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