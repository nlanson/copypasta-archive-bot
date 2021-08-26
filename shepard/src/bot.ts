//Dependencies
import Snoowrap, { SnoowrapOptions, Comment } from 'snoowrap';
import { CommentStream } from 'snoostorm';
import { DatabaseRequest, Err, DbResponse } from './db';
import { log, Level } from './log';


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
        this.streamComments('copypasta'); //Set subreddit to stream comments
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
            if (args.length < 2) return;
            let key: string = args.slice(1, args.length).join(" ");

            //Run through respective pipe process.
            if (args[0] == '!save') {
                await this.save(comment, key);
                return;
            } else if (args[0] == '!send') {
                await this.send(comment, key);
                return;
            } else {
                log(Level.Warning, "Invalid command");
                return;
            }
        });
    }

    private async save(comment: Comment, name: string) {
        log(Level.Info, `Save requested by u/${comment.author.name}`);

        let pasta: string = await this.extractParentContent(comment.parent_id);
        let response: DbResponse = await DatabaseRequest.save(name, pasta);
        if (response.success) 
            await this.reply2thread(comment, "Saved. Use the command " + "` !send "+ `${name}` + " ` to paste");
        else {
            //Filter through the error type and set an appropriate failure message to reply
            log(Level.Error, "Save failed");
            let msg: string;
            switch (+response.err) {
                //Filter through error reasons and set appropriate reply message.
                case Err.DUPLICATE_EXISTS:
                    msg = "Failed to save. Another pasta with the name " + "` " + name + " ` already exists.";
                    break;
                case Err.INVALID_PARENT:
                    msg = "The text you tried to save is invalid.\nPlease retry with a post/comment with text only.";
                    break;
                default:
                    msg = "Unexpected error occured when saving.\nPlease contact u/keijyu if this issue persists.";
            }
            await this.reply2thread(comment, msg);
        }
    }

    private async send(comment: Comment, name: string) {
        log(Level.Info, `Send requested by u/${comment.author.name}`);

        let response: DbResponse = await DatabaseRequest.send(name);
        if (response.success && response.data) 
            await this.reply2thread(comment, response.data);
        else {
            log(Level.Error, "Pasta retrieval failed");
            let msg: string;
            switch (+response.err) {
                case Err.DOES_NOT_EXIST:
                    msg = `The pasta you ordered does not exist. Try order some pizza instead.`;
                    break;
                default:
                    msg = "Unexpected error occured when saving.\nPlease contact u/keijyu if this issue persists.";
            }
            await this.reply2thread(comment, msg);
        }
    }

    //Function that extracts the parent comment or post
    private async extractParentContent(parent: string): Promise<string> {
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
                return '';
        }

        return pasta;
    }

    //Reply to a comment with message.
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