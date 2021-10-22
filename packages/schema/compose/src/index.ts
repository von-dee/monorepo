import { SchemaFile, SchemaResolvers } from "./types";
import { TypeInfo } from "@web3api/schema-parse";

export * from "./types";
export * from "./compose-schema";

export interface SchemaInfo {
  schema?: string;
  typeInfo?: TypeInfo;
}

export interface ComposerOutput {
  [name: string]: SchemaInfo;
  combined: SchemaInfo;
}

export enum ComposerFilter {
  Schema = 1 << 0,
  TypeInfo = 1 << 1,
  All = Schema | TypeInfo,
}

export interface ComposerOptions {
  schemas: {
    [name: string]: SchemaFile;
  };
  resolvers: SchemaResolvers;
  output: ComposerFilter;
}


