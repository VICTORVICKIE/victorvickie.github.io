import type { App } from '$lib'
import Feats from '$lib/windows/Feats.svelte'
import Game from '$lib/windows/Game.svelte'
import Profile from '$lib/windows/Profile.svelte'
import Project from '$lib/windows/Project.svelte'
import Service from '$lib/windows/Service.svelte'
import Skill from '$lib/windows/Skill.svelte'
import type { ComponentType } from 'svelte'

const Windows = new Map<App, ComponentType>([
    ["Feats", Feats],
    ["Profile", Profile],
    ["Project", Project],
    ["Service", Service],
    ["Skill", Skill],
    ["Game", Game]
]);

export default Windows;
