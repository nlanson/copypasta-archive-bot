import chalk from 'chalk';

export enum Level {
    Info,
    Warning,
    Error
}

export function log(level: Level, msg: string) {
    switch (level) {
        case Level.Info:
            console.log(chalk.white.bold('[INFO] ') + chalk.white(msg));
            break;
        case Level.Warning:
            console.log(chalk.yellow.bold('[WARN] ') + chalk.yellow(msg));
            break;
        case Level.Error:
            console.log(chalk.red.bold('[ERR] ') + chalk.red(msg));
            break;
    }
}