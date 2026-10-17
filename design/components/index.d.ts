import type * as React from 'react';

export interface ButtonProps extends React.ButtonHTMLAttributes<HTMLButtonElement> { variant?: 'zap' | 'plain' | 'gum' | 'ink'; size?: 'md' | 'sm'; arrow?: boolean; href?: string }
export declare function Button(props: ButtonProps): React.ReactElement;

export type Tone = 'mint' | 'zap' | 'gum' | 'lilac' | 'plain';
export interface TagProps { tone?: Tone; children?: React.ReactNode }
export declare function Tag(props: TagProps): React.ReactElement;

export interface PillProps { tag?: string; tagTone?: Tone; children?: React.ReactNode }
export declare function Pill(props: PillProps): React.ReactElement;

export interface HighlightProps { variant?: 'sticker' | 'marker'; tone?: 'zap' | 'gum'; children?: React.ReactNode }
export declare function Highlight(props: HighlightProps): React.ReactElement;

export interface InstallOption { label: string; command: string }
export interface InstallBoxProps { options: InstallOption[]; defaultIndex?: number }
export declare function InstallBox(props: InstallBoxProps): React.ReactElement;

export interface CodeBlockProps { lines?: string[]; code?: string; title?: string; meta?: string; dots?: boolean }
export declare function CodeBlock(props: CodeBlockProps): React.ReactElement;

export interface BenchmarkRow { label: string; sub?: string; seconds: number; display?: string; tone?: 'slow' | 'zap' | 'gum' | 'lilac' }
export interface BenchmarkProps { rows: BenchmarkRow[]; title?: string; caption?: string; href?: string }
export declare function Benchmark(props: BenchmarkProps): React.ReactElement;

export interface ToolCardProps { title: string; letter?: string; tone?: 'zap' | 'lilac' | 'gum' | 'mint'; replaces?: string; description?: string; command?: string; status?: string; statusTone?: Tone; tilt?: number }
export declare function ToolCard(props: ToolCardProps): React.ReactElement;

export interface CalloutProps { tone?: 'note' | 'tip' | 'warn'; title?: string; children?: React.ReactNode }
export declare function Callout(props: CalloutProps): React.ReactElement;

export interface RibbonProps { items: string[] }
export declare function Ribbon(props: RibbonProps): React.ReactElement;

export interface BurstProps { children?: React.ReactNode; sub?: string; tone?: 'gum' | 'zap' | 'mint' | 'lilac'; style?: React.CSSProperties }
export declare function Burst(props: BurstProps): React.ReactElement;

export interface StickerPanelProps extends React.HTMLAttributes<HTMLDivElement> { tone?: 'lilac' | 'zap' | 'gum' | 'mint' }
export declare function StickerPanel(props: StickerPanelProps): React.ReactElement;

declare global { interface Window { Pun: { Button: typeof Button; Pill: typeof Pill; Tag: typeof Tag; Highlight: typeof Highlight; InstallBox: typeof InstallBox; CodeBlock: typeof CodeBlock; Benchmark: typeof Benchmark; ToolCard: typeof ToolCard; Callout: typeof Callout; Ribbon: typeof Ribbon; Burst: typeof Burst; StickerPanel: typeof StickerPanel } } }
