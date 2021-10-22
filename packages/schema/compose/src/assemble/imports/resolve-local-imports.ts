import {
  TypeInfo,
  ObjectDefinition,
  visitObjectDefinition,
  DefinitionKind,
  EnumDefinition,
  visitEnumDefinition,
  GenericDefinition,
  isKind,
  ObjectRef,
  EnumRef,
} from "@web3api/schema-parse";
import { LocalImport, SchemaResolver, SchemaResolvers } from "../..";
import { addHeader } from "../../templates/header.mustache";
import { assembleAndParseSchema } from "../assemble-and-parse-schema";
import { EnumOrObject } from "./EnumOrObject";

export const resolveLocalImports = async (
  importsToResolve: LocalImport[],
  resolveSchema: SchemaResolver,
  typeInfo: TypeInfo,
  mutation: boolean,
  resolvers: SchemaResolvers
): Promise<void> => {
  for (const importToResolve of importsToResolve) {
    const { importedTypes, path } = importToResolve;

    // Resolve the schema
    let schema = await resolveSchema(path);

    if (!schema) {
      throw Error(`Unable to resolve schema at "${path}"`);
    }

    // Make sure the schema has the Web3API header
    if (schema.indexOf("### Web3API Header START ###") === -1) {
      schema = addHeader(schema);
    }

    // Parse the schema into TypeInfo
    const localTypeInfo = await assembleAndParseSchema(
      schema,
      path,
      mutation,
      resolvers
    );

    // Keep track of all imported type names
    const typesToImport: Record<string, GenericDefinition> = {};

    for (const importedType of importedTypes) {
      if (importedType === "Query" || importedType === "Mutation") {
        throw Error(
          `Importing query types from local schemas is prohibited. Tried to import from ${path}.`
        );
      } else {
        let type: GenericDefinition | undefined;
        let visitorFunc: Function;

        if (
          localTypeInfo.objectTypes.findIndex(
            (type) => type.type === importedType
          ) > -1
        ) {
          visitorFunc = visitObjectDefinition;
          type = localTypeInfo.objectTypes.find(
            (type) => type.type === importedType
          );
        } else {
          visitorFunc = visitEnumDefinition;
          type = localTypeInfo.enumTypes.find(
            (type) => type.type === importedType
          );
        }

        if (!type) {
          throw Error(
            `Cannot find type "${importedType}" in the schema at ${path}.\nFound: [ ${localTypeInfo.objectTypes.map(
              (type) => type.type + " "
            )}]`
          );
        }

        typesToImport[type.type] = type;

        const findImport = (
          def: GenericDefinition,
          rootTypes: EnumOrObject[]
        ) => {
          // Skip objects that we've already processed
          if (typesToImport[def.type]) {
            return def;
          }

          // Find the ObjectDefinition
          const idx = rootTypes.findIndex((obj) => obj.type === def.type);

          if (idx === -1) {
            throw Error(
              `resolveLocalImports: Cannot find the requested type within the TypeInfo.\n` +
                `Type: ${def.type}\nTypeInfo: ${JSON.stringify(localTypeInfo)}`
            );
          }

          const objectDefinition = rootTypes[idx];

          if (!visitedTypes[objectDefinition.type]) {
            if (objectDefinition.kind === DefinitionKind.Object) {
              visitedTypes[objectDefinition.type] = true;
              visitType(objectDefinition);
            }
          }

          typesToImport[def.type] = {
            ...objectDefinition,
            name: null,
            required: null,
          };
          return def;
        };

        const visitedTypes: Record<string, boolean> = {};

        const visitType = (type: GenericDefinition) => {
          visitorFunc(type, {
            enter: {
              ObjectRef: (def: ObjectRef) => {
                return findImport(def, [
                  ...localTypeInfo.objectTypes,
                  ...localTypeInfo.importedObjectTypes,
                ]);
              },
              EnumRef: (def: EnumRef) => {
                return findImport(def, [
                  ...localTypeInfo.enumTypes,
                  ...localTypeInfo.importedEnumTypes,
                ]);
              },
            },
          });
        };

        visitedTypes[type.type] = true;
        visitType(type);
      }
    }

    // Add all imported types into the aggregate TypeInfo
    for (const importType of Object.keys(typesToImport)) {
      if (isKind(typesToImport[importType], DefinitionKind.Object)) {
        if (
          typeInfo.objectTypes.findIndex((def) => def.type === importType) ===
          -1
        ) {
          typeInfo.objectTypes.push(
            typesToImport[importType] as ObjectDefinition
          );
        }
      } else {
        if (
          typeInfo.enumTypes.findIndex((def) => def.type === importType) === -1
        ) {
          typeInfo.enumTypes.push(typesToImport[importType] as EnumDefinition);
        }
      }
    }
  }
}
