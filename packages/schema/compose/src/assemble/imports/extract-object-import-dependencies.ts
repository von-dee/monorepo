import {
  TypeInfo,
  ImportedObjectDefinition,
  TypeInfoTransforms,
  visitObjectDefinition,
  DefinitionKind,
  ImportedEnumDefinition,
  InterfaceImplementedDefinition,
  ObjectRef,
  EnumRef,
} from "@web3api/schema-parse";
import { appendNamespace } from "./append-namespace";
import { EnumOrObject } from "./EnumOrObject";
import { ImportedEnumOrObject } from "./ImportedEnumOrObject";
import { ImportMap } from "./ImportMap";
import { Namespaced } from "./Namespaced";

// A transformation that converts all object definitions into
// imported object definitions
export const extractObjectImportDependencies = (
  importsFound: ImportMap,
  rootTypeInfo: TypeInfo,
  namespace: string,
  uri: string
): TypeInfoTransforms => {
  const findImport = (
    type: string,
    namespaceType: string,
    rootTypes: EnumOrObject[],
    importedTypes: ImportedEnumOrObject[],
    kind: DefinitionKind
  ): ImportedEnumOrObject & Namespaced => {
    // Find this type's ObjectDefinition in the root type info
    let idx = rootTypes.findIndex((obj) => obj.type === type);
    let obj = undefined;

    if (idx === -1) {
      idx = importedTypes.findIndex((obj) => obj.type === type);
    } else {
      obj = rootTypes[idx];
    }

    if (idx === -1) {
      throw Error(
        `extractObjectImportDependencies: Cannot find the dependent type within the root type info.\n` +
          `Type: ${type}\nTypeInfo: ${JSON.stringify(
            rootTypeInfo
          )}\n${namespace}\n${JSON.stringify(Object.keys(importsFound))}`
      );
    } else if (obj === undefined) {
      obj = importedTypes[idx];
    }

    // Create the new ImportedObjectDefinition
    return {
      ...obj,
      name: null,
      required: null,
      type: namespaceType,
      __namespaced: true,
      kind,
      uri,
      namespace,
      nativeType: type,
    };
  };

  return {
    enter: {
      ObjectRef: (def: ObjectRef & Namespaced) => {
        if (def.__namespaced) {
          return def;
        }

        const type = def.type;

        const namespaceType = appendNamespace(namespace, type);

        if (!importsFound[namespaceType]) {
          // Find the import
          const importFound = findImport(
            type,
            namespaceType,
            rootTypeInfo.objectTypes,
            rootTypeInfo.importedObjectTypes,
            DefinitionKind.ImportedObject
          ) as ImportedObjectDefinition;

          // Keep track of it
          importsFound[importFound.type] = importFound;

          // Traverse this newly added object
          visitObjectDefinition(importFound, {
            ...extractObjectImportDependencies(
              importsFound,
              rootTypeInfo,
              namespace,
              uri
            ),
          });
        }

        return def;
      },
      InterfaceImplementedDefinition: (
        def: InterfaceImplementedDefinition & Namespaced
      ) => {
        if (def.__namespaced) {
          return def;
        }

        const type = def.type;

        const namespaceType = appendNamespace(namespace, type);

        if (!importsFound[namespaceType]) {
          // Find the import
          const importFound = findImport(
            type,
            namespaceType,
            rootTypeInfo.objectTypes,
            rootTypeInfo.importedObjectTypes,
            DefinitionKind.ImportedObject
          ) as ImportedObjectDefinition;

          // Keep track of it
          importsFound[importFound.type] = importFound;

          // Traverse this newly added object
          visitObjectDefinition(importFound, {
            ...extractObjectImportDependencies(
              importsFound,
              rootTypeInfo,
              namespace,
              uri
            ),
          });
        }

        return def;
      },
      EnumRef: (def: EnumRef & Namespaced) => {
        if (def.__namespaced) {
          return def;
        }

        const namespaceType = appendNamespace(namespace, def.type);
        if (!importsFound[namespaceType]) {
          // Find the import
          const importFound = findImport(
            def.type,
            namespaceType,
            rootTypeInfo.enumTypes,
            rootTypeInfo.importedEnumTypes,
            DefinitionKind.ImportedEnum
          ) as ImportedEnumDefinition;

          // Keep track of it
          importsFound[importFound.type] = importFound;
        }

        return def;
      },
    },
  };
};