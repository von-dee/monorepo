import { parseExternalImports, parseLocalImports } from "../parse";
import { ExternalImport, LocalImport, SchemaResolvers, SYNTAX_REFERENCE } from "../types";
import { addQueryImportsDirective } from "./add-query-imports-directive";
import { TYPE_NAME_REGEX } from "./constants/TYPE-NAME-REGEX";
import {
  TypeInfo,
  header,
} from "@web3api/schema-parse";
import { renderSchema } from "../render";
import { resolveExternalImports } from "./imports/resolve-external-imports";
import { resolveLocalImports } from "./imports/resolve-local-imports";
import { parseInterfaces } from "./interfaces/parse-interfaces";
import { resolveInterfaces } from "./interfaces/resolve-interfaces";

export async function assembleSchema(
  schema: string,
  schemaPath: string,
  mutation: boolean,
  resolvers: SchemaResolvers
): Promise<string> {
  const importKeywordCapture = /^#+["{3}]*import\s/gm;
  const externalImportCapture = /#+["{3}]*import\s*{([^}]+)}\s*into\s*(\w+?)\s*from\s*[\"'`]([^\"'`\s]+)[\"'`]/g;
  const localImportCapture = /#+["{3}]*import\s*{([^}]+)}\s*from\s*[\"'`]([^\"'`\s]+)[\"'`]/g;

  const keywords = [...schema.matchAll(importKeywordCapture)];
  const externalImportStatements = [...schema.matchAll(externalImportCapture)];
  const localImportStatments = [...schema.matchAll(localImportCapture)];
  const totalStatements =
    externalImportStatements.length + localImportStatments.length;

  if (keywords.length !== totalStatements) {
    throw Error(
      `Invalid import statement found in file ${schemaPath}.\nPlease use one of the following syntaxes...\n${SYNTAX_REFERENCE}`
    );
  }

  const interfaceCapture = new RegExp(
    `type\\s+${TYPE_NAME_REGEX}\\s+implements\\s([^{]*){`,
    "g"
  );
  const implementInterfaceStatments = [...schema.matchAll(interfaceCapture)];

  const implementationsWithInterfaces = parseInterfaces(
    implementInterfaceStatments
  );

  const externalImportsToResolve: ExternalImport[] = parseExternalImports(
    externalImportStatements,
    mutation
  );

  const localImportsToResolve: LocalImport[] = parseLocalImports(
    localImportStatments,
    schemaPath
  );

  const subTypeInfo: TypeInfo = {
    objectTypes: [],
    queryTypes: [],
    enumTypes: [],
    importedEnumTypes: [],
    importedObjectTypes: [],
    importedQueryTypes: [],
  };

  const externalImports = await resolveExternalImports(
    externalImportsToResolve,
    resolvers.external,
    subTypeInfo
  );

  await resolveLocalImports(
    localImportsToResolve,
    resolvers.local,
    subTypeInfo,
    mutation,
    resolvers
  );

  // Remove all import statements
  let newSchema = schema
    .replace(externalImportCapture, "")
    .replace(localImportCapture, "");

  // Remove all non documentation comments
  newSchema = newSchema.replace(/#[^\n]*\n/g, "");

  // Add the @imports directive
  newSchema = addQueryImportsDirective(newSchema, externalImports, mutation);

  //Combine the new schema with the subTypeInfo
  newSchema = header + newSchema + renderSchema(subTypeInfo, false);

  newSchema = resolveInterfaces(newSchema, implementationsWithInterfaces);

  //Replace types that have empty curly brackets with types that have no curly brackets
  //because GraphQL parser doesn't support empty curly brackets but supports no curly brackets
  newSchema = newSchema.replace(
    new RegExp(`(type\\s+${TYPE_NAME_REGEX})[^{]*{\\s*}`, "g"),
    "$1"
  );

  return newSchema;
};