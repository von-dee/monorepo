import { appendNamespace } from './append-namespace';
import {
  TypeInfo,
  parseSchema,
  ObjectDefinition,
  ImportedObjectDefinition,
  QueryDefinition,
  visitObjectDefinition,
  visitQueryDefinition,
  ImportedQueryDefinition,
  DefinitionKind,
  visitImportedQueryDefinition,
  visitImportedObjectDefinition,
  ImportedEnumDefinition,
  EnumDefinition,
  visitEnumDefinition,
  visitImportedEnumDefinition,
} from "@web3api/schema-parse";
import { ImportMap } from './ImportMap';
import { namespaceTypes } from './namespace-types';
import { ExternalImport, SchemaResolver } from '../..';
import { extractObjectImportDependencies } from './extract-object-import-dependencies';

export const resolveExternalImports = async (
  importsToResolve: ExternalImport[],
  resolveSchema: SchemaResolver,
  typeInfo: TypeInfo
): Promise<string[]> => {
  // Keep track of all imported object type names
  const typesToImport: ImportMap = {};

  for (const importToResolve of importsToResolve) {
    const { uri, namespace, importedTypes } = importToResolve;

    // Resolve the schema
    const schema = await resolveSchema(uri);

    if (!schema) {
      throw Error(`Unable to resolve schema at "${uri}"`);
    }

    // Parse the schema into TypeInfo
    const extTypeInfo = parseSchema(schema);

    // For each imported type to resolve
    for (const importedType of importedTypes) {
      let extTypes: (
        | QueryDefinition
        | ObjectDefinition
        | EnumDefinition
      )[] = [];
      let visitorFunc: Function;
      let trueTypeKind: DefinitionKind;

      // If it's a query type
      if (importedType === "Query" || importedType === "Mutation") {
        extTypes = extTypeInfo.queryTypes;
        visitorFunc = visitQueryDefinition;
        trueTypeKind = DefinitionKind.ImportedQuery;
      } else if (
        importedType.endsWith("_Query") ||
        importedType.endsWith("_Mutation")
      ) {
        throw Error(
          `Cannot import an import's imported query type. Tried to import ${importedType} from ${uri}.`
        );
      } else {
        if (
          extTypeInfo.objectTypes.findIndex(
            (def) => def.type === importedType
          ) > -1
        ) {
          extTypes = extTypeInfo.objectTypes;
          visitorFunc = visitObjectDefinition;
          trueTypeKind = DefinitionKind.ImportedObject;
        } else if (
          extTypeInfo.importedObjectTypes.findIndex(
            (def) => def.type === importedType
          ) > -1
        ) {
          extTypes = extTypeInfo.importedObjectTypes;
          visitorFunc = visitObjectDefinition;
          trueTypeKind = DefinitionKind.ImportedObject;
        } else if (
          extTypeInfo.importedEnumTypes.findIndex(
            (def) => def.type === importedType
          ) > -1
        ) {
          extTypes = extTypeInfo.importedEnumTypes;
          visitorFunc = visitEnumDefinition;
          trueTypeKind = DefinitionKind.ImportedEnum;
        } else {
          extTypes = extTypeInfo.enumTypes;
          visitorFunc = visitEnumDefinition;
          trueTypeKind = DefinitionKind.ImportedEnum;
        }
      }

      // Find the type's definition in the schema's TypeInfo
      const type = extTypes.find((type) => type.type === importedType);

      if (!type) {
        throw Error(
          `Cannot find type "${importedType}" in the schema at ${uri}.\nFound: [ ${extTypes.map(
            (type) => type.type + " "
          )}]`
        );
      }

      const namespacedType = appendNamespace(namespace, importedType);

      // Continue if we've already imported this type
      if (typesToImport[namespacedType]) {
        continue;
      }

      // Append the base type to our TypeInfo
      typesToImport[namespacedType] = {
        ...type,
        name: null,
        required: null,
        type: namespacedType,
        kind: trueTypeKind,
        namespace,
        __namespaced: true,
        uri,
        nativeType: type.type,
      };

      // Extract all object dependencies
      visitorFunc(
        type,
        extractObjectImportDependencies(
          typesToImport,
          extTypeInfo,
          namespace,
          uri
        )
      );
    }

    // Add all imported types into the aggregate TypeInfo
    for (const importName of Object.keys(typesToImport)) {
      const importType = typesToImport[importName];
      let destArray:
        | ImportedObjectDefinition[]
        | ImportedQueryDefinition[]
        | ImportedEnumDefinition[];
      let append;

      if (importType.kind === DefinitionKind.ImportedObject) {
        destArray = typeInfo.importedObjectTypes;
        append = () => {
          const importDef = importType as ImportedObjectDefinition;
          // Namespace all object types
          typeInfo.importedObjectTypes.push(
            visitImportedObjectDefinition(importDef, namespaceTypes(namespace))
          );
        };
      } else if (importType.kind === DefinitionKind.ImportedQuery) {
        destArray = typeInfo.importedQueryTypes;
        append = () => {
          const importDef = importType as ImportedQueryDefinition;
          // Namespace all object types
          typeInfo.importedQueryTypes.push(
            visitImportedQueryDefinition(importDef, namespaceTypes(namespace))
          );
        };
      } else if (importType.kind === DefinitionKind.ImportedEnum) {
        destArray = typeInfo.importedEnumTypes;
        append = () => {
          typeInfo.importedEnumTypes.push(
            visitImportedEnumDefinition(
              importType as ImportedEnumDefinition,
              namespaceTypes(namespace)
            )
          );
        };
      } else {
        throw Error(
          `resolveExternalImports: This should never happen, unknown kind.\n${JSON.stringify(
            importType,
            null,
            2
          )}`
        );
      }

      const found =
        destArray.findIndex(
          (
            def:
              | ImportedObjectDefinition
              | ImportedQueryDefinition
              | ImportedEnumDefinition
          ) => def.type === importType.type
        ) > -1;

      if (!found) {
        append();
      }
    }
  }

  return Promise.resolve(Object.keys(typesToImport));
};