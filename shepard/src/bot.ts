//Dependencies
import Snoowrap, { SnoowrapOptions, Submission, Comment } from 'snoowrap';
import { CommentStream } from 'snoostorm';
import axios from 'axios';

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

        comments.on('item', (comment) => {
            if ( this.connectedAt > comment.created_utc ) return;
            let content: String = comment.body;
            if (!content.includes("!shepard")) return;

            /*
                From here, the bot needs to parse the arguement given by the user and determine whether it 
                is requesting a send or a save. 

                A save will look something like this:
                    !shepard save "name"
                This will save the contents of the post or comment above under the name provided so it can 
                be requested later.

                A send will look something like this:
                    !shepard send "name"
                This will find the copy pasta saved under the name provided and reply with it.
            */
            let args: Array<String> = content.split(" ");

            if (args[1] == 'save') {
                let parent: string = comment.parent_id;
                let name: String = args[2];
                this.save(parent, name);
            } else if (args[1] == 'send') {
                let pasta = this.send(args[2]);
                //Bot to reply with the pasta contents.
            }
        });
    }

    private async save(parent: string, name: String) {
        let content: any;
        switch (true) {
            case parent[1] == "1":
                content = await this.reddit.getComment(parent).body;
                break;
            case parent[1] == "3":
                content = await this.reddit.getSubmission(parent).selftext;
                break;
            default:
                console.log("Invalid parent.");
                return;
        }

        console.log(content);
        let post = {
            name: name,
            value: content
        };

        //Change localhost to the Server IP for production.
        axios
            .get(`http://localhost:8000/save/${process.env.auth_key}/${post.name}/${post.value}`)
            .then((res: any) => {
                //Need to extract response status and handle any errors.
                console.log(res);
            })
            .catch((error: any) => {
                console.error(error)
            });
    }

    private async send(name: String) {
        /* 
            Make a request to the back end http://ipaddress:8000/send/process.env.auth_key/name/

            This will return a JSend with the pasta attached as the data payload.

            Return this pasta for the bot to reply.
        */
    }
}