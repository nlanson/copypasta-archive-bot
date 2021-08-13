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
            pollTime: 3000
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
                log(Level.Info, `Save requested by u/${comment.author.name}`);
                let saved = await this.save(comment.parent_id, args.slice(1, args.length).join(" "));
                if (!saved) log(Level.Error, "Save failed");
                return;
            } else if (args[0] == '!send') {
                log(Level.Info, `Send requested by u/${comment.author.name}`);
                let pasta = await this.send(args.slice(1, args.length).join(" "));
                if (pasta.length != 0) comment.reply(pasta).catch((err: any) => log(Level.Error, err.message));
                else log(Level.Error, "Pasta retrieval failed");
                return;
            } else {
                log(Level.Warning, "Invalid command");
                return
            }
        });
    }

    //Function that extracts the parent comment and sends the save request to the database.
    private async save(parent: string, name: String): Promise<Boolean> {
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
            .get(`http://host.docker.internal:8000/save/${process.env.auth_key}/${name}/${pasta}`)
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
    private async send(name: String): Promise<string> {
        let pasta: string = '';

        await axios
            .get(`http://host.docker.internal:8000/send/${process.env.auth_key}/${name}`)
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
}