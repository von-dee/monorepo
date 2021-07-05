/* eslint-disable @typescript-eslint/naming-convention */
import { generateBinding } from "./bindings";
import { getRelativePath, findCommonTypes, extendCommonTypes } from "./utils";

import { transformTypeInfo, TypeInfo } from "@web3api/schema-parse";

export * from "./utils";

export type TargetLanguage = "wasm-as";

export type OutputEntry = FileEntry | DirectoryEntry | TemplateEntry;

export interface FileEntry {
  type: "File";
  name: string;
  data: string;
}

export interface DirectoryEntry {
  type: "Directory";
  name: string;
  data: OutputEntry[];
}

export interface TemplateEntry {
  type: "Template";
  name: string;
  data: string;
}

export interface OutputDirectory {
  entries: OutputEntry[];
}

export interface BindOutput {
  query?: OutputDirectory;
  mutation?: OutputDirectory;
  subscription?: OutputDirectory;
}

export interface BindModuleOptions {
  typeInfo: TypeInfo;
  outputDirAbs: string;
}

export interface BindOptions {
  language: TargetLanguage;
  query?: BindModuleOptions;
  mutation?: BindModuleOptions;
  subscription?: BindModuleOptions;
}

export function bindSchema(options: BindOptions): BindOutput {
  const { query, mutation, subscription, language } = options;

  // If at least two modules are present,
  // determine which types are shared between them,
  // and add the __common & __commonPath properties
  const modules = [query, mutation, subscription].filter(
    (v): v is BindModuleOptions => v !== undefined
  );
  if (modules.length > 1) {
    // Find all common types
    const commonTypes = findCommonTypes(modules.map((m) => m.typeInfo));

    if (commonTypes.length) {
      modules[0].typeInfo = transformTypeInfo(
        modules[0].typeInfo,
        extendCommonTypes(commonTypes)
      );

      // Compute the __commonPath
      const commonPath =
        getRelativePath(modules[1].outputDirAbs, modules[0].outputDirAbs) +
        "/common";

      for (let i = 1; i < modules.length; i++) {
        modules[i].typeInfo = {
          ...transformTypeInfo(
            modules[i].typeInfo,
            extendCommonTypes(commonTypes, commonPath)
          ),
          __commonPath: commonPath,
        } as TypeInfo;
      }
    }
  }

  return {
    query: query ? generateBinding(language, query.typeInfo) : undefined,
    mutation: mutation
      ? generateBinding(language, mutation.typeInfo)
      : undefined,
    subscription: subscription
      ? generateBinding(language, subscription.typeInfo)
      : undefined,
  };
}
