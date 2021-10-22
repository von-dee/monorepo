import { assembleAndParseSchema } from './assemble/assemble-and-parse-schema';
import { renderSchema } from "./render";
import { TypeInfo, combineTypeInfo } from "@web3api/schema-parse";
import { ComposerOptions, ComposerOutput, ComposerFilter, SchemaInfo } from './index';

export async function composeSchema(
  options: ComposerOptions
): Promise<ComposerOutput> {
  const { schemas, resolvers } = options;
  const typeInfos: {
    [name: string]: TypeInfo;
  } = {};

  if (Object.keys(schemas).length === 0) {
    throw Error("No schema provided");
  }

  for (const name of Object.keys(schemas)) {
    const schema = schemas[name];

    typeInfos[name] = await assembleAndParseSchema(
      schema.schema,
      schema.absolutePath,
      name === "mutation",
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
  const typeInfoNames = Object.keys(typeInfos);

  for (const name of typeInfoNames) {
    const typeInfo = typeInfos[name];
    output[name] = createSchemaInfo(typeInfo);
  }

  if (typeInfoNames.length > 1) {
    const combinedTypeInfo = combineTypeInfo(
      typeInfoNames.map((name) => typeInfos[name])
    );

    output.combined = createSchemaInfo(combinedTypeInfo);
  } else {
    output.combined = output[typeInfoNames[0]];
  }

  return output;
}
