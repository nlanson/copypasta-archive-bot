//Dependencies
import Snoowrap, { SnoowrapOptions, Submission } from 'snoowrap';
import { CommentStream } from 'snoostorm';

export class Bot {
    private reddit: Snoowrap;
    private connectedAt: number;

    constructor() {
        this.reddit = new Snoowrap(this.loadBotCredentials());
        this.connectedAt = Date.now() / 1000;
        this.streamComments();
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
            subreddit: "copypasta",
            pollTime: 3000
        });

        comments.on('item', (comment) => {
            if ( this.connectedAt > comment.created_utc ) return;
            let commentContent: String = comment.body.toLowerCase();
            if (!commentContent.includes("!shepard")) return;

            console.log("DETECTED");
        });
    }
}