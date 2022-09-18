import * as ESTree from 'estree';

declare module 'acorn' {
    type ExtendObject<T> = {
        [K in keyof T]: ExtendNode<T[K]>;
    };
    type WithStartEnd<T> = T extends ESTree.Node | ESTree.Comment
        ? { start: number; end: number }
        : unknown;
    export type ExtendNode<T> = T extends object ? ExtendObject<T> & WithStartEnd<T> : T;
    export function parse(s: string, o: Options): ExtendNode<ESTree.Program>;
    
    export type AcornComment = Omit<Comment, 'type'> & {
        type: 'Line' | 'Block';
    };
}