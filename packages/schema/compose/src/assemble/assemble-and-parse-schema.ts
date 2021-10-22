import { SchemaResolvers } from "..";
import { assembleSchema } from "./assemble-schema";
import {
  TypeInfo,
  parseSchema
} from "@web3api/schema-parse";

export const assembleAndParseSchema = async (
  schema: string,
  schemaPath: string,
  mutation: boolean,
  resolvers: SchemaResolvers
): Promise<TypeInfo> => {
  const assembledSchema = assembleSchema(
    schema,
    schemaPath,
    mutation,
    resolvers
  );

  const typeInfo = parseSchema(assembledSchema);

  return typeInfo;
};