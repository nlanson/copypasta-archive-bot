//Dependencies
import Snoowrap, { SnoowrapOptions, Submission, Comment } from 'snoowrap';
import { CommentStream } from 'snoostorm';
import axios from 'axios';

interface Payload {
    status: String,
    data: any
};

/*
    Todo:
        - Multi word pasta names
        - Better logging
*/

export class Bot {
    private reddit: Snoowrap;
    private connectedAt: number;

    constructor() {
        this.reddit = new Snoowrap(this.loadBotCredentials());
        this.connectedAt = Date.now() / 1000;
        this.streamComments();
        console.log("Bot connected.");
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
            console.log("Environment credentials not detected.");
            process.exit(1);
        }
    }

    private streamComments() {
        const comments = new CommentStream(this.reddit, {
            subreddit: "u_keijyu",
            pollTime: 3000
        });

        comments.on('item', async (comment) => {
            if ( this.connectedAt > comment.created_utc ) return;
            let content: String = comment.body;
            if (!content.includes("!save") && !content.includes("!send")) return;

            let args: Array<String> = content.split(" ");

            if (args[0] == '!save') {
                console.log("Save requested...")
                let saved = await this.save(comment.parent_id, args[1]);
                if (!saved) console.log("    -> Save failed");
                return;
            } else if (args[0] == '!send') {
                console.log("Send requested...")
                let pasta = await this.send(args[1]);
                if (pasta.length != 0) comment.reply(pasta);
                else console.log("    -> Pasta retrieval failed.");
                return;
            } else {
                console.log("Invalid comment.");
                return
            }
        });
    }

    //Function that extracts the parent comment and sends the save request to the database.
    private async save(parent: string, name: String): Promise<Boolean> {
        let pasta: any;
        switch (true) {
            case parent[1] == "1":
                pasta = await this.reddit.getComment(parent).body;
                break;
            case parent[1] == "3":
                pasta = await this.reddit.getSubmission(parent).selftext;
                break;
            default:
                console.log("Invalid parent.");
                return false;
        }

        //Change localhost to the Server IP for production.
        let success: Boolean = false;
        await axios
            .get(`http://localhost:8000/save/${process.env.auth_key}/${name}/${pasta}`)
            .then((res: any) => {
                let payload: Payload = res.data;
                if (payload.status == 'success') success = true;
                else {
                    console.log(payload.data)
                    success = false;
                }
            })
            .catch((error: any) => {
                console.log(error);
                success = false;
            });
        return success;
    }

    //Function that fetches the reeusted pasta from the database.
    private async send(name: String): Promise<string> {
        let pasta: string = '';

        await axios
            .get(`http://localhost:8000/send/${process.env.auth_key}/${name}`)
            .then((res: any) => {
                let payload: Payload = res.data;
                if (payload.status == 'success') {
                    pasta = payload.data;
                }
            })
            .catch((error: any) => {
                console.error(error)
            });
        
        return pasta;
    }
}