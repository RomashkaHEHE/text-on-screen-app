export const PROTOCOL_VERSION = 1;

export type SyncMessage = { type: "sync"; update: string };
export type HelloMessage = { type: "hello"; protocolVersion: number; minimumProtocolVersion: number; minimumClientVersion: string; sessionId: string; role: "editor" | "viewer" };
export type ServerMessage = SyncMessage | HelloMessage | { type: "awareness"; state: unknown } | { type: "error"; code: string };
