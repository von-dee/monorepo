import { QueryType, SchemaFile, SchemaResolvers } from "./types";
import { resolveImportsAndParseSchemas } from "./resolve";
import { renderSchema } from "./render";

import { combineTypeInfo, TypeInfo } from "@web3api/schema-parse";

export * from "./types";

export interface SchemaInfo {
  schema?: string;
  typeInfo?: TypeInfo;
}

export interface ComposerOutput {
  query?: SchemaInfo;
  mutation?: SchemaInfo;
  subscription?: SchemaInfo;
  combined: SchemaInfo;
}

export enum ComposerFilter {
  Schema = 1 << 0,
  TypeInfo = 1 << 1,
  All = Schema | TypeInfo,
}

export interface ComposerOptions {
  schemas: {
    query?: SchemaFile;
    mutation?: SchemaFile;
    subscription?: SchemaFile;
  };
  resolvers: SchemaResolvers;
  output: ComposerFilter;
}

export async function composeSchema(
  options: ComposerOptions
): Promise<ComposerOutput> {
  const { schemas, resolvers } = options;
  const { query, mutation, subscription } = schemas;
  const typeInfos: {
    query?: TypeInfo;
    mutation?: TypeInfo;
    subscription?: TypeInfo;
  } = {};

  if (query && query.schema) {
    typeInfos.query = await resolveImportsAndParseSchemas(
      query.schema,
      query.absolutePath,
      QueryType.Query,
      resolvers
    );
  }

  if (mutation && mutation.schema) {
    typeInfos.mutation = await resolveImportsAndParseSchemas(
      mutation.schema,
      mutation.absolutePath,
      QueryType.Mutation,
      resolvers
    );
  }

  if (subscription && subscription.schema) {
    typeInfos.subscription = await resolveImportsAndParseSchemas(
      subscription.schema,
      subscription.absolutePath,
      QueryType.Subscription,
      resolvers
    );
  }

  const output: ComposerOutput = {
    combined: {},
  };
  const includeSchema = options.output & ComposerFilter.Schema;
  const includeTypeInfo = options.output & ComposerFilter.TypeInfo;
  const createSchemaInfo = (typeInfo: TypeInfo): SchemaInfo => ({
    schema: includeSchema ? renderSchema(typeInfo, true) : undefined,
    typeInfo: includeTypeInfo ? typeInfo : undefined,
  });

  if (typeInfos.query) {
    output.query = createSchemaInfo(typeInfos.query);
  }

  if (typeInfos.mutation) {
    output.mutation = createSchemaInfo(typeInfos.mutation);
  }

  if (typeInfos.subscription) {
    output.subscription = createSchemaInfo(typeInfos.subscription);
  }

  const typeInfosUsed: TypeInfo[] = [
    typeInfos.query,
    typeInfos.mutation,
    typeInfos.subscription,
  ].filter((v: TypeInfo | undefined): v is TypeInfo => v !== undefined);

  if (typeInfosUsed.length > 1) {
    const combinedTypeInfo = combineTypeInfo(typeInfosUsed);
    output.combined = createSchemaInfo(combinedTypeInfo);
  } else if (typeInfos.query && output.query) {
    output.combined = output.query;
  } else if (typeInfos.mutation && output.mutation) {
    output.combined = output.mutation;
  } else if (typeInfos.subscription && output.subscription) {
    output.combined = output.subscription;
  }

  return output;
}
