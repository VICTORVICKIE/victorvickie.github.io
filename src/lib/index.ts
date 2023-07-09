import { derived, writable } from 'svelte/store';

export const Apps = ['Feats', 'Skill', 'Profile', 'Project', 'Service', 'Game'] as const;

export type App = (typeof Apps)[number];

export type AppIcon = {
    [key in Exclude<App, "Game">]: string;
};

export const AppIcon: AppIcon = {
    Feats: 'game-icons:diamond-trophy',
    Skill: 'game-icons:skills',
    Profile: 'game-icons:astronaut-helmet',
    Project: 'game-icons:auto-repair',
    Service: 'game-icons:gear-hammer'
};

export type State = "close" | "open" | "minimize" | "maximize" | "restore"

export type AppState = {
    [key in App]: State;
};


export const app_state = writable<AppState>({
    Feats: "close",
    Skill: "close",
    Profile: "close",
    Project: "close",
    Service: "close",
    Game: "close"
});
