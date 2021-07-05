/* eslint-disable @typescript-eslint/naming-convention */
import {
  GenericDefinition,
  TypeInfo,
  TypeInfoTransforms,
} from "@web3api/schema-parse";

export function findCommonTypes(typeInfos: TypeInfo[]): string[] {
  const types: Record<string, boolean> = {};

  const addType = (def: GenericDefinition) => {
    types[def.type] = true;
  };

  typeInfos[0].objectTypes.forEach(addType);
  typeInfos[0].enumTypes.forEach(addType);
  typeInfos[0].importedEnumTypes.forEach(addType);
  typeInfos[0].importedObjectTypes.forEach(addType);
  typeInfos[0].importedQueryTypes.forEach(addType);

  const commonTypes: string[] = [];

  const tryAddCommonType = (def: GenericDefinition) => {
    if (types[def.type]) {
      commonTypes.push(def.type);
    }
  };

  for (let i = 1; i < typeInfos.length; i++) {
    typeInfos[i].objectTypes.forEach(tryAddCommonType);
    typeInfos[i].enumTypes.forEach(tryAddCommonType);
    typeInfos[i].importedEnumTypes.forEach(tryAddCommonType);
    typeInfos[i].importedObjectTypes.forEach(tryAddCommonType);
    typeInfos[i].importedQueryTypes.forEach(tryAddCommonType);
  }

  return commonTypes;
}

export function extendCommonTypes(
  commonTypes: string[],
  commonPath?: string
): TypeInfoTransforms {
  const commonExtension = (type: string) =>
    commonTypes.includes(type)
      ? {
          __common: true,
          __commonPath: commonPath || null,
          __commonImport: !!commonPath,
        }
      : {
          __common: null,
          __commonPath: commonPath || null,
          __commonImport: false,
        };

  return {
    enter: {
      GenericDefinition: (def: GenericDefinition) => ({
        ...def,
        ...commonExtension(def.type),
      }),
    },
  };
}
